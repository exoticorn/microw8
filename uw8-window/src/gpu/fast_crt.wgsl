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

fn row_factor(offset: f32) -> f32 {
    return 1.0 / (1.0 + offset * offset * 16.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base = round(in.tex_coords) - vec2<f32>(0.5);
    let frac = in.tex_coords - base;
    
    let top_factor = row_factor(frac.y);
    let bottom_factor = row_factor(frac.y - 1.0);
    
    let v = base.y + bottom_factor / (bottom_factor + top_factor);
    
    let u = in.tex_coords.x;
    
    return textureSample(screen_texture, linear_sampler, vec2<f32>(u, v) / vec2<f32>(320.0, 240.0)) * (top_factor + bottom_factor) * 2.0;
}

