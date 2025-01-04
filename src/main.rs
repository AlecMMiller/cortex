mod buffer;
mod color;
mod rectangle;
mod render;
mod setup;
mod sidebar;
mod size;
mod state;

use std::sync::Arc;

use buffer::{DisplayInfoBuffer, RectBuffer, VertexBundle};
use color::PaletteBuffer;
use setup::{get_surface_config, PipelineContext, RenderContext};
use state::AppState;
use tracing::{debug, info, span, Level};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Arc<Window>) {
    info!("Initializing graphics");
    let render_context = RenderContext::new(window.clone()).await;

    debug!("Creating buffers");
    let mut display_info_buffer = DisplayInfoBuffer::new(&render_context.device, &window);
    let rectangle_buffer = RectBuffer::new(&render_context.device);
    let palette_buffer = PaletteBuffer::new(&render_context.device);
    let vertex_bundle = VertexBundle::new(&render_context.device);
    let pipeline = PipelineContext::new(
        &render_context,
        &display_info_buffer,
        &palette_buffer,
        &rectangle_buffer,
    );
    let mut surface_config = get_surface_config(&window, &render_context);

    let mut state = AppState::new(&window);

    let main_window_id = window.id();

    info!("Starting event loop");
    event_loop
        .run(move |event, target| {
            let needs_redraw = match event {
                Event::WindowEvent {
                    window_id,
                    event: window_event,
                } if window_id == main_window_id => {
                    let span = span!(Level::INFO, "window_event", event = debug(&window_event));
                    let _enter = span.enter();
                    match window_event {
                        WindowEvent::RedrawRequested => {
                            info!("Drawing screen");
                            render::render(
                                &window,
                                &render_context,
                                &pipeline,
                                &mut state,
                                &rectangle_buffer,
                                &vertex_bundle,
                                &palette_buffer,
                                &display_info_buffer,
                            );
                            false
                        }
                        WindowEvent::CloseRequested => {
                            info!("Stopping the app");
                            target.exit();
                            false
                        }
                        WindowEvent::Resized(new_size) => {
                            info!("Resizing the window");
                            state.resize(new_size);
                            display_info_buffer.set_size(&new_size);
                            surface_config.width = new_size.width;
                            surface_config.height = new_size.height;
                            render_context
                                .surface
                                .configure(&render_context.device, &surface_config);
                            true
                        }
                        WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            inner_size_writer: _,
                        } => {
                            info!("Scale factor changed");
                            state.set_scale_factor(scale_factor);
                            true
                        }
                        _ => false,
                    }
                }
                _ => false,
            };

            if needs_redraw {
                info!("Requesting redraw");
            };
        })
        .unwrap();
}

pub fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to initialize tracing subscriber");

    info!("Application started");

    let event_loop = EventLoop::new().unwrap();
    let builder = winit::window::WindowBuilder::new().with_title("Cortex");
    let window = Arc::new(builder.build(&event_loop).unwrap());
    info!("Window created");
    pollster::block_on(run(event_loop, window));
}
