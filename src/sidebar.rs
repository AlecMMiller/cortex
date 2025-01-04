use tracing::debug;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

use crate::color::Palette;
use crate::rectangle::Rectangle;
use crate::size::Bounds;

const SIDEBAR_WIDTH: f32 = 40.0;
const SIDEBAR_HEIGHT: f32 = 20.0;

pub struct Sidebar {
    scale_factor: f32,
    pub inner: Bounds,
    size: PhysicalSize<f32>,
    pub rects: Vec<Rectangle>,
}

impl Sidebar {
    fn recalculate(&mut self) {
        self.inner.origin.x = SIDEBAR_WIDTH * self.scale_factor;
        self.inner.origin.y = SIDEBAR_HEIGHT * self.scale_factor;

        self.inner.size.width = self.size.width - self.inner.origin.x;
        self.inner.size.height = self.size.height - self.inner.origin.y;

        self.rects = Vec::new();

        let sidebar_rect =
            Rectangle::new(PhysicalPosition::new(0.0, 0.0), self.size).color(Palette::Crust);

        let inner_rect =
            Rectangle::new_from_bounds(&self.inner).radius_tl(10.0 * self.scale_factor);

        debug!("content bounds are {0:?}", self.inner);

        self.rects.push(sidebar_rect);
        self.rects.push(inner_rect);
    }

    #[tracing::instrument(level="debug" skip(self))]
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size.height = new_size.height as f32;
        self.size.width = new_size.width as f32;

        self.recalculate();
    }

    #[tracing::instrument(level="debug" skip(self))]
    pub fn rescale(&mut self, scale: f64) {
        self.scale_factor = scale as f32;

        self.recalculate();
    }

    #[tracing::instrument(level="debug" skip(window))]
    pub fn new(window: &Window) -> Self {
        let window_height = window.inner_size().height as f32;
        let window_width = window.inner_size().width as f32;
        let scale_factor = window.scale_factor() as f32;

        let mut new = Self {
            inner: Bounds {
                origin: PhysicalPosition { x: 0.0, y: 0.0 },
                size: PhysicalSize {
                    width: 0.0,
                    height: 0.0,
                },
            },
            size: PhysicalSize {
                height: window_height,
                width: window_width,
            },
            scale_factor,
            rects: Vec::new(),
        };

        new.recalculate();

        new
    }
}
