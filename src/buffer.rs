use bytemuck::{Pod, Zeroable};
use encase::ShaderType;
use glam::Vec4;
use tracing::debug;
use wgpu::{
    util::DeviceExt as _, Buffer, BufferDescriptor, BufferUsages, Device, Queue, RenderPass,
};
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Debug, ShaderType)]
pub struct DisplayInfo {
    pub size: Vec4,
}

pub struct DisplayInfoBuffer {
    pub buffer: Buffer,
    pub data: DisplayInfo,
}

impl DisplayInfoBuffer {
    pub fn new(device: &Device, window: &Window) -> Self {
        debug!("Creating display info buffer");
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<DisplayInfo>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let data = DisplayInfo {
            size: [
                window.inner_size().width as f32,
                window.inner_size().height as f32,
                1.0,
                0.0,
            ]
            .into(),
        };

        Self { buffer, data }
    }

    pub fn set_size(&mut self, size: &PhysicalSize<u32>) {
        self.data.size = [size.width as f32, size.height as f32, 1.0, 0.0].into();
    }

    fn as_wgsl_bytes(&self) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(&self.data)?;
        Ok(buffer.into_inner())
    }

    pub fn write_to_queue(&self, queue: &Queue) {
        queue.write_buffer(
            &self.buffer,
            0,
            &self.as_wgsl_bytes().expect(
                "Error in encase translating AppState \
                    struct to WGSL bytes.",
            ),
        );
    }
}

pub struct RectBuffer {
    pub buffer: Buffer,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Rectangle {
    position: [f32; 2],
    size: [f32; 2],
    color: u32,
    _pad2: [f32; 3],
    corner_radius: [f32; 4],
}

impl RectBuffer {
    pub fn new(device: &Device) -> Self {
        debug!("Creating rectangle buffer");
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<Rectangle>() as u64 * 1000,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self { buffer }
    }
}

pub struct VertexBundle {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pub num_indices: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

impl VertexBundle {
    pub fn new(device: &Device) -> Self {
        debug!("Creating vertex_buffer");
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        debug!("Creating index buffer");
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn add_to_pass(&self, render_pass: &mut RenderPass) {
        debug!("Adding vertices to pass");
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    }
}
