struct VertexInput {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) pixel_position: vec2<f32>,
    @location(1) rect_origin: vec2<f32>,
    @location(2) rect_size: vec2<f32>,
    @location(3) rect_center: vec2<f32>,
    @location(4) rect_corner: vec2<f32>,
    @location(5) corner_radius: vec4<f32>,
    @location(6) color_index: u32,
};

struct RectangleUniform {
    origin: vec2<f32>,
    size: vec2<f32>,
    color_index: u32,
    corner_radius: vec4<f32>
};

struct DisplayUniform {
   size: vec2<f32>,
};

fn distance_from_rect(pixel_pos: vec2<f32>, rect_center: vec2<f32>, rect_corner: vec2<f32>, corner_radius: f32) -> f32 {
    let p = pixel_pos - rect_center;
    let q = abs(p) - rect_corner + corner_radius;
    return length(max(q, vec2(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - corner_radius;
}

@group(0) @binding(0) // 1.
var<uniform> display: DisplayUniform;

@group(0) @binding(1) // 1.
var<uniform> palette: array<vec4<f32>, 7>;

@group(0) @binding(2) // 1.
var<storage> rectangle: array<RectangleUniform>;

@vertex
fn vert_main(
    model: VertexInput,
) -> VertexOutput {
    let rect = rectangle[model.instance_index];

    let pixel_pos = model.position.xy * vec2(rect.size) + rect.origin.xy;
    let device_pos = pixel_pos / vec2(display.size.xy) * vec2(2.0, -2.0) + vec2(-1.0, 1.0);

    var out: VertexOutput;

    out.position = vec4(device_pos, 0.0, 1.0);
    out.pixel_position = pixel_pos;
    out.rect_origin = rect.origin.xy;
    out.rect_size = rect.size;
    out.rect_corner = rect.size / 2.0;
    out.rect_center = rect.origin.xy + out.rect_corner;
    out.corner_radius = rect.corner_radius;
    out.color_index = rect.color_index;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var corner_radius: f32;

    if in.position.y >= in.rect_center.y {
        //border_corner.y -= in.border_bottom;
        if in.position.x >= in.rect_center.x {
            corner_radius = in.corner_radius[3];
        } else {
            corner_radius = in.corner_radius[2];
        }
    } else {
        if in.position.x >= in.rect_center.x {
            corner_radius = in.corner_radius[1];
        } else {
            corner_radius = in.corner_radius[0];
        }
    }

    let shape_distance = distance_from_rect(in.position.xy, in.rect_center, in.rect_corner, corner_radius);

    var color = palette[in.color_index];

    if corner_radius > 0 {
        color.a *= 1.0 - smoothstep(-0.75, -0.1, shape_distance);
    }

    return color;
}
