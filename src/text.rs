use std::sync::{Arc, Mutex};

use glyphon::{
    Attrs, Buffer, Cache, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea,
    TextAtlas, TextBounds, TextRenderer, Viewport,
};
use tracing::debug;
use wgpu::{Device, MultisampleState, Queue, RenderPass};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

use crate::{setup::SWAPHCHAIN_FORMAT, size::Bounds};

pub struct TextBlock {
    buffer: Buffer,
    context: Arc<Mutex<TextContext>>,
    scale_factor: f32,
    origin: PhysicalPosition<f32>,
}

impl TextBlock {
    #[tracing::instrument(skip(context, origin))]
    pub fn new(
        context: Arc<Mutex<TextContext>>,
        scale_factor: f32,
        origin: PhysicalPosition<f32>,
    ) -> Self {
        let font_system = &mut context.lock().unwrap().font_system;
        let buffer = glyphon::Buffer::new(font_system, Metrics::new(32.0, 32.0));

        Self {
            buffer,
            context: context.clone(),
            scale_factor,
            origin,
        }
    }

    #[tracing::instrument(skip(self, text), fields(length = text.len()))]
    pub fn set_text(&mut self, text: &str) {
        debug!("Setting text");

        let font_system = &mut self.context.lock().unwrap().font_system;

        self.buffer.set_text(
            font_system,
            text,
            Attrs::new().metrics(Metrics::new(32.0, 32.0)),
            Shaping::Advanced,
        );

        debug!("Shaping");
        self.buffer.shape_until_scroll(font_system, true);
    }

    pub fn set_bounds(&mut self, bounds: &Bounds, scale_factor: f32) {
        debug!("Size being set");
        self.scale_factor = scale_factor;
        let inner_bounds = bounds.inset(PhysicalSize {
            width: 40.0 * scale_factor,
            height: 0.0,
        });
        self.origin = inner_bounds.origin;

        let font_system = &mut self.context.lock().unwrap().font_system;

        self.buffer.set_size(
            font_system,
            Some(inner_bounds.size.width / self.scale_factor),
            Some(inner_bounds.size.height / self.scale_factor),
        );
    }

    pub fn get_text_area(&self) -> TextArea {
        TextArea {
            buffer: &self.buffer,
            left: self.origin.x,
            top: self.origin.y,
            scale: self.scale_factor,
            bounds: TextBounds::default(),
            default_color: glyphon::Color::rgb(205, 214, 244),
            custom_glyphs: &[],
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
        let mut text_atlas = TextAtlas::with_color_mode(
            device,
            queue,
            &cache,
            SWAPHCHAIN_FORMAT,
            glyphon::ColorMode::Web,
        );
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
