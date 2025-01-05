use std::usize;

use glam::Vec4;
use lyon::tessellation::VertexBuffers;
use wgpu::{
    include_wgsl, Buffer, BufferDescriptor, BufferUsages, Device, Queue, RenderPass,
    RenderPipeline, COPY_BUFFER_ALIGNMENT,
};
use winit::dpi::PhysicalSize;

use super::gpu_types::{GpuGlobals, GpuPrimitive, GpuTransform, GpuVertex};

pub struct SvgRenderer {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline: RenderPipeline,
    transforms_ssbo: Buffer,
    prims_ssbo: Buffer,
    globals_ubo: Buffer,
    bind_group: wgpu::BindGroup,
    num_indices: usize,
    canvas_size: PhysicalSize<u32>,
}

fn next_copy_buffer_size(size: u64) -> u64 {
    let align_mask = COPY_BUFFER_ALIGNMENT - 1;
    ((size.next_power_of_two() + align_mask) & !align_mask).max(COPY_BUFFER_ALIGNMENT)
}

impl SvgRenderer {
    pub fn set_canvas_size(&mut self, size: &PhysicalSize<u32>) {
        self.canvas_size = *size;
    }

    pub fn new(device: &Device) -> Self {
        let vertex_buffer_size = next_copy_buffer_size(32768);

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("svg vertices"),
            size: vertex_buffer_size,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer_size = next_copy_buffer_size(32768);

        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("svg indices"),
            size: index_buffer_size,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let prim_buffer_byte_size = (1024 * std::mem::size_of::<GpuPrimitive>()) as u64;

        let prims_ssbo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Prims ssbo"),
            size: prim_buffer_byte_size,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let transform_buffer_byte_size = (1024 * std::mem::size_of::<GpuTransform>()) as u64;

        let transforms_ssbo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Transforms ssbo"),
            size: transform_buffer_byte_size,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let globals_buffer_byte_size = std::mem::size_of::<GpuGlobals>() as u64;

        let globals_ubo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Globals ubo"),
            size: globals_buffer_byte_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(globals_buffer_byte_size),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(prim_buffer_byte_size),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(transform_buffer_byte_size),
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(globals_ubo.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(prims_ssbo.as_entire_buffer_binding()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        transforms_ssbo.as_entire_buffer_binding(),
                    ),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
            label: None,
        });

        let shader_module = device.create_shader_module(include_wgsl!("./shaders.wgsl"));

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GpuVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x2,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            offset: 8,
                            format: wgpu::VertexFormat::Uint32,
                            shader_location: 1,
                        },
                    ],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                polygon_mode: wgpu::PolygonMode::Fill,
                front_face: wgpu::FrontFace::Ccw,
                strip_index_format: None,
                cull_mode: None,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        };

        let pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

        Self {
            vertex_buffer,
            index_buffer,
            pipeline,
            transforms_ssbo,
            prims_ssbo,
            globals_ubo,
            bind_group,
            num_indices: 0,
            canvas_size: PhysicalSize::new(0, 0),
        }
    }

    pub fn write_mesh(&mut self, queue: &Queue, mesh: &VertexBuffers<GpuVertex, u32>) {
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&mesh.vertices));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&mesh.indices));

        self.num_indices = mesh.indices.len();
    }

    pub fn write_transforms(&self, queue: &Queue, transforms: &Vec<GpuTransform>) {
        queue.write_buffer(&self.transforms_ssbo, 0, bytemuck::cast_slice(transforms));
    }

    pub fn write_primitives(&self, queue: &Queue, primitives: &Vec<GpuPrimitive>) {
        queue.write_buffer(&self.prims_ssbo, 0, bytemuck::cast_slice(primitives));
    }

    pub fn render(&self, pass: &mut RenderPass) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        pass.draw_indexed(0..(self.num_indices as u32), 0, 0..1);
    }

    pub fn write_size(&self, queue: &Queue) {
        let data = GpuGlobals {
            size: Vec4::new(
                self.canvas_size.width as f32,
                self.canvas_size.height as f32,
                0.0,
                0.0,
            ),
        };
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(&data).unwrap();

        queue.write_buffer(&self.globals_ubo, 0, &buffer.into_inner());
    }
}
