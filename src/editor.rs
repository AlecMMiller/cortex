use std::sync::{Arc, Mutex};

use glyphon::TextArea;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{
    size::Bounds,
    text::{TextBlock, TextContext},
};

pub struct Editor {
    content: TextBlock,
    context: Arc<Mutex<TextContext>>,
}

impl Editor {
    pub fn new(context: &Arc<Mutex<TextContext>>) -> Self {
        let mut content = TextBlock::new(context.clone(), 1.0, PhysicalPosition::new(0.0, 0.0));

        content.set_text("Foo");

        content.set_bounds(
            &Bounds {
                origin: PhysicalPosition { x: 100.0, y: 100.0 },
                size: PhysicalSize {
                    width: 500.0,
                    height: 500.0,
                },
            },
            1.0,
        );

        Self {
            content,
            context: context.clone(),
        }
    }

    pub fn get_text_areas(&self) -> Vec<TextArea> {
        let mut ret = Vec::new();

        let only = self.content.get_text_area();
        ret.push(only);

        ret
    }
}
