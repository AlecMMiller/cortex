use std::sync::{Arc, Mutex};

use tracing::info;
use wgpu::RenderPass;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    buffer::DisplayInfoBuffer,
    color::{PaletteBuffer, MOCHA},
    editor::Editor,
    rectangle::Rectangle,
    setup::RenderContext,
    sidebar::Sidebar,
    svg::SvgRenderer,
    text::TextContext,
};

enum Content {
    Editor(Editor),
}

pub struct AppState {
    scale_factor: f64,
    resize_event: bool,
    palette_change: bool,
    text: Arc<Mutex<TextContext>>,
    content: Content,
    sidebar: Sidebar,
}

impl<'a> AppState {
    pub fn new(window: &Window, text: TextContext) -> Self {
        let sidebar = Sidebar::new(window);

        let text = Mutex::new(text);
        let text = Arc::new(text);

        let editor = Editor::new(&text);

        Self {
            scale_factor: window.scale_factor(),
            resize_event: false,
            palette_change: true,
            sidebar,
            content: Content::Editor(editor),
            text,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.resize_event = true;
        self.sidebar.resize(size);

        //match &mut self.content {
        //    Content::Editor(editor) => {
        //        editor.set_bounds(&self.sidebar.inner, self.scale_factor as f32)
        //    }
        //}
    }

    pub fn get_rects(&self) -> Vec<Rectangle> {
        let mut res = Vec::new();

        res.extend(self.sidebar.rects.clone());

        //match &self.content {
        //    Content::Editor(editor) => res.extend(editor.rects.clone()),
        //}

        res
    }

    pub fn render(&self, pass: &mut RenderPass) {
        let text = self.text.lock().unwrap();
        text.render(pass);
    }

    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
        self.sidebar.rescale(scale_factor);
    }

    pub fn clear(&mut self) {
        self.palette_change = false;
        self.resize_event = false;
    }

    pub fn prepare(
        &mut self,
        window: &Window,
        context: &RenderContext,
        palette_buffer: &PaletteBuffer,
        display_info_buffer: &DisplayInfoBuffer,
        svg_context: &SvgRenderer,
    ) {
        if self.palette_change {
            info!("Writing to palette_buffer");
            palette_buffer.write_to_queue(&context.queue, MOCHA);
        }

        {
            let mut text_context = self.text.lock().unwrap();

            if self.resize_event {
                info!("Writing new size info");
                display_info_buffer.write_to_queue(&context.queue);
                text_context.resize(&context.queue, window);
                svg_context.write_size(&context.queue);
            }

            let mut text_areas = Vec::new();

            match &self.content {
                Content::Editor(editor) => text_areas.extend(editor.get_text_areas()),
            }

            text_context.prepare(&context.device, &context.queue, text_areas);
        }

        //match &mut self.content {
        //    Content::Editor(editor) => {
        //        editor.prepare(
        //            window,
        //            context,
        //            &self.sidebar.inner,
        //            self.scale_factor as f32,
        //        );
        //   }
        //}
    }
}
