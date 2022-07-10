struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = (1.0 - f32(vertex_index)) * 3.0;
    let y = f32(vertex_index & 1u) * 3.0 - 1.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>((x + 1.0) * 160.0, (y + 1.0) * 120.0);
    return out;
}

@group(0) @binding(0) var framebuffer_texture: texture_2d<u32>;
@group(0) @binding(1) var palette_texture: texture_1d<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texel = vec2<i32>(floor(in.tex_coords));
    let index = textureLoad(framebuffer_texture, texel, 0).r;
    return textureLoad(palette_texture, i32(index), 0);
}