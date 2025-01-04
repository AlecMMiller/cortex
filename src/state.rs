use tracing::info;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

use crate::{
    buffer::DisplayInfoBuffer,
    color::{PaletteBuffer, MOCHA},
    rectangle::Rectangle,
    setup::RenderContext,
    sidebar::Sidebar,
    text::TextContext,
};

pub struct AppState {
    scale_factor: f64,
    resize_event: bool,
    palette_change: bool,
    //active: ActiveElement,
    cursor: Option<PhysicalPosition<f64>>,
    sidebar: Sidebar,
    //content: Content<'a>,
}

impl AppState {
    pub fn new(window: &Window) -> Self {
        let sidebar = Sidebar::new(window);

        Self {
            scale_factor: window.scale_factor(),
            resize_event: false,
            palette_change: true,
            cursor: None,
            sidebar,
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

    pub fn set_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
        self.sidebar.rescale(scale_factor);
    }

    pub fn clear(&mut self) {
        self.palette_change = false;
        self.resize_event = false;
    }

    pub fn prepare(
        &self,
        window: &Window,
        context: &RenderContext,
        palette_buffer: &PaletteBuffer,
        display_info_buffer: &DisplayInfoBuffer,
        text_context: &mut TextContext,
    ) {
        if self.palette_change {
            info!("Writing to palette_buffer");
            palette_buffer.write_to_queue(&context.queue, MOCHA);
        }

        if self.resize_event {
            info!("Writing new size info");
            display_info_buffer.write_to_queue(&context.queue);
            text_context.resize(&context.queue, window);
        }

        let text_areas = Vec::new();
        text_context.prepare(&context.device, &context.queue, text_areas);

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
