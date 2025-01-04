use tracing::{debug, info};
use winit::window::Window;

use crate::{
    buffer::{DisplayInfoBuffer, RectBuffer, VertexBundle},
    color::{make_color, PaletteBuffer},
    setup::{PipelineContext, RenderContext},
    state::AppState,
};

#[tracing::instrument(skip(
    context,
    pipeline,
    vertexes,
    state,
    rect_buffer,
    palette,
    display_info
))]
pub fn render(
    window: &Window,
    context: &RenderContext,
    pipeline: &PipelineContext,
    state: &mut AppState,
    rect_buffer: &RectBuffer,
    vertexes: &VertexBundle,
    palette: &PaletteBuffer,
    display_info: &DisplayInfoBuffer,
) {
    let frame = context.surface.get_current_texture().unwrap();

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    state.prepare(window, context, palette, display_info);

    let mut encoder = context
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let bg = make_color(0xFF0000, 1.0);
        debug!("Beginning render pass");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(bg),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        let rects = state.get_rects();
        let num_instances = rects.len() as u32;

        context
            .queue
            .write_buffer(&rect_buffer.buffer, 0, bytemuck::cast_slice(&rects));

        render_pass.set_pipeline(&pipeline.pipeline);
        vertexes.add_to_pass(&mut render_pass);
        render_pass.set_bind_group(0, Some(&pipeline.bind_group), &[]);
        render_pass.draw_indexed(0..vertexes.num_indices, 0, 0..num_instances);
    }
    debug!("Submitting encoder queue");
    context.queue.submit(Some(encoder.finish()));
    debug!("Presenting frame");
    frame.present();
    info!("Render complete");

    state.clear();
}
