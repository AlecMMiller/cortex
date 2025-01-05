use bytemuck::{Pod, Zeroable};
use encase::ShaderType;
use glam::Vec4;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GpuTransform {
    pub data0: [f32; 4],
    pub data1: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GpuPrimitive {
    pub transform: u32,
    pub color: u32,
    pub scale: f32,
    pub _pad: u32,
}

impl GpuPrimitive {
    pub fn new(transform_idx: u32, color: usvg::Color, alpha: f32) -> Self {
        GpuPrimitive {
            transform: transform_idx,
            color: ((color.red as u32) << 24)
                + ((color.green as u32) << 16)
                + ((color.blue as u32) << 8)
                + (alpha * 255.0) as u32,
            scale: 1.0,
            _pad: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GpuVertex {
    pub position: [f32; 2],
    pub prim_id: u32,
}

#[derive(Debug, ShaderType)]
pub struct GpuGlobals {
    pub size: Vec4,
}
