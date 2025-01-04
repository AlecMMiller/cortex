use std::time::{Duration, Instant};

use glam::bool;
use tracing::{debug, info, warn};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton},
};

pub struct CursorState {
    location: Option<PhysicalPosition<f64>>,
    is_pressed: bool,
    press_count: u8,
    last_press: Option<Instant>,
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            location: None,
            is_pressed: false,
            press_count: 0,
            last_press: None,
        }
    }

    pub fn moved(&mut self, position: PhysicalPosition<f64>) -> bool {
        self.location = Some(position);

        false
    }

    pub fn mouse_input(&mut self, element_state: ElementState, button: MouseButton) -> bool {
        let Some(location) = self.location else {
            warn!("Mouse click detected but no cursor location known");
            return false;
        };

        match self.last_press {
            Some(last_press) => {
                let time_since = last_press.elapsed();

                if time_since > Duration::from_millis(500) {
                    debug!(elapsed = debug(time_since), "Not a double click");
                    self.last_press = None;
                    self.press_count = 0;
                }
            }
            None => {}
        };

        match button {
            MouseButton::Left => match element_state {
                ElementState::Pressed => {
                    self.last_press = Some(Instant::now());
                    self.is_pressed = true;
                    if self.press_count == 0 {
                        info!(x = location.x, y = location.y, "Left click");
                    }

                    false
                }
                ElementState::Released => {
                    self.last_press = Some(Instant::now());
                    self.is_pressed = false;
                    self.press_count += 1;
                    match self.press_count {
                        1 => {
                            info!(x = location.x, y = location.y, "Single click");
                            false
                        }
                        2 => {
                            info!(x = location.x, y = location.y, "Double click");
                            false
                        }
                        3 => {
                            info!(x = location.x, y = location.y, "Triple click");
                            false
                        }
                        _ => false,
                    }
                }
            },
            _ => false,
        }
    }
}
