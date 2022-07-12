struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Uniforms {
    texture_scale: vec4<f32>,
}

@group(0) @binding(2) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let i = in_vertex_index / 3u + in_vertex_index % 3u;
    let x = 0.0 + f32(i % 2u) * 320.0;
    let y = 0.0 + f32(i / 2u) * 240.0;
    out.clip_position = vec4<f32>((vec2<f32>(x, y) - vec2<f32>(160.0, 120.0)) * uniforms.texture_scale.xy, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x, y);
    return out;
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var linear_sampler: sampler;

fn aa_tex_coord(c: f32) -> f32 {
    let low = c - uniforms.texture_scale.z * 0.5;
    let high = c + uniforms.texture_scale.z * 0.5;
    let base = floor(low);
    let center = base + 0.5;
    let next = base + 1.0;
    if high > next {
        return center + (high - next) / (high - low);
    } else {
        return center;
    }
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(screen_texture, linear_sampler, vec2<f32>(aa_tex_coord(in.tex_coords.x), aa_tex_coord(in.tex_coords.y)) / vec2<f32>(320.0, 240.0));
}
