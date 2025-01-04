use std::sync::Arc;

use glyphon::{
    Cache, FontSystem, Resolution, SwashCache, TextArea, TextAtlas, TextRenderer, Viewport,
};
use tracing::{debug, info, warn};
use wgpu::{
    BindGroup, ColorTargetState, CompositeAlphaMode, Device, MultisampleState, PowerPreference,
    PresentMode, Queue, RenderPass, RenderPipeline, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};
use winit::window::Window;

use crate::{
    buffer::{DisplayInfoBuffer, RectBuffer, Vertex},
    color::PaletteBuffer,
};

#[derive(Debug)]
pub struct RenderContext {
    pub device: wgpu::Device,
    pub surface: wgpu::Surface<'static>,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

fn get_power_preference() -> PowerPreference {
    let Ok(manager) = battery::Manager::new() else {
        warn!("Could not get battery status, assuming non-battery and using high performance mode");
        return PowerPreference::HighPerformance;
    };
    let Ok(mut batteries) = manager.batteries() else {
        info!("Could not find any batteries, assuming non-battery and using high performance mode");
        return PowerPreference::HighPerformance;
    };
    let Some(first_battery) = batteries.next() else {
        info!(
            "Could not get status of battery, assuming non-battery and using high performance mode"
        );
        return PowerPreference::HighPerformance;
    };
    let Ok(first_battery) = first_battery else {
        warn!(
            "Could not get status of battery, assuming non-battery and using high performance mode"
        );
        return PowerPreference::HighPerformance;
    };

    let state = first_battery.state();

    match state {
        battery::State::Full | battery::State::Charging => {
            info!("Battery state is {state}, using high performance mode");
            PowerPreference::HighPerformance
        }
        _ => {
            info!("Battery state is {state}, using low performance mode");
            PowerPreference::LowPower
        }
    }
}

impl RenderContext {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        info!("Screen size is {size:?}");

        debug!("Starting WGPU instance");
        let instance = wgpu::Instance::default();
        debug!("Creating surface");
        let surface = instance.create_surface(window.clone()).unwrap();
        let power_preference = get_power_preference();
        debug!("Requesting adapter");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        debug!("Creating device and render queue");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .expect("Failed to create device/queue");

        Self {
            adapter,
            device,
            surface,
            queue,
        }
    }
}

const SWAPHCHAIN_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;

pub struct PipelineContext {
    pub bind_group: BindGroup,
    pub pipeline: RenderPipeline,
}

impl PipelineContext {
    pub fn new(
        context: &RenderContext,
        display_info: &DisplayInfoBuffer,
        palette: &PaletteBuffer,
        rect: &RectBuffer,
    ) -> Self {
        debug!("Creating pipeline context");
        let bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &display_info.buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &palette.buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                            buffer: &rect.buffer,
                            offset: 0,
                            size: None,
                        }),
                    },
                ],
            });

        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    // (4)
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        debug!("Loading shaders");
        let shader = context
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders.wgsl"));

        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vert_main"),
                    compilation_options: Default::default(),
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState {
                        format: SWAPHCHAIN_FORMAT.into(),
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent::OVER,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        Self {
            bind_group,
            pipeline,
        }
    }
}

pub struct TextContext {
    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: Viewport,
    text_atlas: TextAtlas,
    text_renderer: TextRenderer,
}

impl TextContext {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        debug!("Setting up text context");
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut text_atlas = TextAtlas::new(device, queue, &cache, SWAPHCHAIN_FORMAT);
        let text_renderer =
            TextRenderer::new(&mut text_atlas, &device, MultisampleState::default(), None);

        Self {
            font_system,
            swash_cache,
            viewport,
            text_atlas,
            text_renderer,
        }
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        debug!("Rendering text");
        self.text_renderer
            .render(&self.text_atlas, &self.viewport, render_pass)
            .unwrap();
    }

    pub fn resize(&mut self, queue: &Queue, window: &Window) {
        self.viewport.update(
            &queue,
            Resolution {
                width: window.inner_size().width,
                height: window.inner_size().height,
            },
        );
    }

    pub fn prepare(&mut self, device: &Device, queue: &Queue, text_areas: Vec<TextArea>) {
        let count = text_areas.len();
        debug!(count, "Preparing text areas");
        self.text_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.text_atlas,
                &mut self.viewport,
                text_areas,
                &mut self.swash_cache,
            )
            .unwrap();
    }
}

pub fn get_surface_config(window: &Window, context: &RenderContext) -> SurfaceConfiguration {
    debug!("Configuring surface");
    let surface_config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: SWAPHCHAIN_FORMAT,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: PresentMode::Fifo,
        alpha_mode: CompositeAlphaMode::Opaque,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    context.surface.configure(&context.device, &surface_config);

    surface_config
}
