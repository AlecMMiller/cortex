use glam::bool;
use winit::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Debug)]
pub struct Bounds {
    pub origin: PhysicalPosition<f32>,
    pub size: PhysicalSize<f32>,
}

pub trait Contains<T> {
    fn contains(&self, position: PhysicalPosition<T>) -> bool;
}

impl Bounds {
    pub fn inset(&self, amount: PhysicalSize<f32>) -> Self {
        let inner_origin = PhysicalPosition {
            x: self.origin.x + amount.width,
            y: self.origin.y + amount.height,
        };

        let inner_size = PhysicalSize {
            width: self.size.width - amount.width * 2.0,
            height: self.size.height - amount.height * 2.0,
        };

        Self {
            origin: inner_origin,
            size: inner_size,
        }
    }

    pub fn get_outer_y(&self) -> f32 {
        self.origin.y + self.size.height
    }

    pub fn get_outer_x(&self) -> f32 {
        self.origin.x + self.size.width
    }
}

impl Contains<f32> for Bounds {
    fn contains(&self, position: PhysicalPosition<f32>) -> bool {
        self.origin.x <= position.x
            && self.origin.y <= position.y
            && self.get_outer_y() >= position.y
            && self.get_outer_x() >= position.x
    }
}

impl Contains<f64> for Bounds {
    fn contains(&self, position: PhysicalPosition<f64>) -> bool {
        self.origin.x as f64 <= position.x
            && self.origin.y as f64 <= position.y
            && self.get_outer_y() as f64 >= position.y
            && self.get_outer_x() as f64 >= position.x
    }
}
