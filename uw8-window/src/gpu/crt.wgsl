struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Uniforms {
    texture_scale: vec4<f32>,
}

@group(0) @binding(1) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let i = in_vertex_index / 3u + in_vertex_index % 3u;
    let x = -1.0 + f32(i % 2u) * 322.0;
    let y = -1.0 + f32(i / 2u) * 242.0;
    out.clip_position = vec4<f32>((vec2<f32>(x, y) - vec2<f32>(160.0, 120.0)) / uniforms.texture_scale.xy, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x, y);
    return out;
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;

fn sample_pixel(coords: vec2<i32>, offset: vec4<f32>) -> vec3<f32> {
    let is_outside = any(vec2<u32>(coords) >= vec2<u32>(320u, 240u));
    if(is_outside) {
        return vec3<f32>(0.0);
    } else {
        let f = max(vec4<f32>(0.008) / offset - vec4<f32>(0.0024), vec4<f32>(0.0));
        return textureLoad(screen_texture, coords, 0).rgb * (f.x + f.y + f.z + f.w);
    }
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixelf = floor(in.tex_coords);
    let o = vec2<f32>(0.5) - (in.tex_coords - pixelf);
    let pixel = vec2<i32>(pixelf);
    
    let offset_x = o.xxxx + vec4<f32>(-0.125, 0.375, 0.125, -0.375) * uniforms.texture_scale.z;
    let offset_y = o.yyyy + vec4<f32>(-0.375, -0.125, 0.375, 0.125) * uniforms.texture_scale.z;
    
    var offset_x0 = max(abs(offset_x + vec4<f32>(-1.0)) - vec4<f32>(0.5), vec4<f32>(0.0));
    var offset_x1 = max(abs(offset_x) - vec4<f32>(0.5), vec4<f32>(0.0));
    var offset_x2 = max(abs(offset_x + vec4<f32>(1.0)) - vec4<f32>(0.5), vec4<f32>(0.0));
    
    offset_x0 = offset_x0 * offset_x0;
    offset_x1 = offset_x1 * offset_x1;
    offset_x2 = offset_x2 * offset_x2;
    
    var offset_yr = offset_y + vec4<f32>(-1.0);
    offset_yr = vec4<f32>(0.02) + offset_yr * offset_yr;
    
    var acc = sample_pixel(pixel + vec2<i32>(-1, -1), offset_x0 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(0, -1), offset_x1 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(1, -1), offset_x2 + offset_yr);

    offset_yr = vec4<f32>(0.02) + offset_y * offset_y;
    
    acc = acc + sample_pixel(pixel + vec2<i32>(-1, 0), offset_x0 + offset_yr);
    acc = acc + sample_pixel(pixel, offset_x1 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(1, 0), offset_x2 + offset_yr);

    offset_yr = offset_y + vec4<f32>(1.0);
    offset_yr = vec4<f32>(0.02) + offset_yr * offset_yr;
    
    acc = acc + sample_pixel(pixel + vec2<i32>(-1, 1), offset_x0 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(0, 1), offset_x1 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(1, 1), offset_x2 + offset_yr);

    return vec4<f32>(acc, 1.0);
}