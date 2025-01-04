use glyphon::{
    Cache, FontSystem, Resolution, SwashCache, TextArea, TextAtlas, TextRenderer, Viewport,
};
use tracing::debug;
use wgpu::{Device, MultisampleState, Queue, RenderPass};
use winit::window::Window;

use crate::setup::SWAPHCHAIN_FORMAT;

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
