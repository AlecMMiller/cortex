use bytemuck::{Pod, Zeroable};
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{color::Palette, size::Bounds};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Rectangle {
    position: [f32; 2],
    size: [f32; 2],
    color: u32,
    _pad2: [f32; 3],
    corner_radius: [f32; 4],
}

impl Rectangle {
    pub fn new(position: PhysicalPosition<f32>, size: PhysicalSize<f32>) -> Self {
        Self {
            position: [position.x, position.y],
            color: Palette::Base as u32,
            size: [size.width, size.height],
            corner_radius: [0.0, 0.0, 0.0, 0.0],
            _pad2: [0.0, 0.0, 0.0],
        }
    }

    pub fn new_from_bounds(bounds: &Bounds) -> Self {
        Self::new(bounds.origin, bounds.size)
    }

    pub fn color(mut self, color: Palette) -> Self {
        self.color = color as u32;
        self
    }

    pub fn radius_tl(mut self, radius: f32) -> Self {
        self.corner_radius[0] = radius;
        self
    }
}
