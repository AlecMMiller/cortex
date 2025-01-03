struct AppState {
    aspect: f32,
}


@group(0)
@binding(0)
var<uniform> app_state: AppState;

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var thing = length(in.uv) / 2.0;
    var cut = step(0.5, thing);

    return vec4<f32>(cut, cut, cut, 1.0);
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vert_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {

    const TRI_VERTICES = array(
        vec4(-1.0, -1.0, 0., 1.),
        vec4(3.0, -1.0, 0., 1.),
        vec4(-1.0, 3.0, 0., 1.),
    );
    var out: VertexOutput;
    //let x = f32(1 - i32(in_vertex_index)) * 0.5;
    //let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    //out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.clip_position = TRI_VERTICES[in_vertex_index];
    out.vert_pos = out.clip_position.xyz;
    out.uv = out.clip_position.xy;
    return out;
}
