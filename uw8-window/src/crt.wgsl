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
    let x = (1.0 - f32(in_vertex_index)) * 3.0;
    let y = f32(in_vertex_index & 1u) * 3.0 - 1.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(x, y) * uniforms.texture_scale.xy + vec2<f32>(160.0, 120.0);
    return out;
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;

fn sample_pixel(coords: vec2<i32>, offset: vec4<f32>) -> vec3<f32> {
    let is_outside = any(vec2<u32>(coords) >= vec2<u32>(320u, 240u));
    if(is_outside) {
        return vec3<f32>(0.0);
    } else {
        let f = max(vec4<f32>(0.01) / offset - vec4<f32>(0.003), vec4<f32>(0.0));
        return textureLoad(screen_texture, coords, 0).rgb * (f.x + f.y + f.z + f.w);
    }
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel = floor(in.tex_coords);
    let o = vec2<f32>(0.5) - (in.tex_coords - pixel);
    let pixel = vec2<i32>(pixel);
    
    if(pixel.x < -1 || pixel.y < -1 || pixel.x > 320 || pixel.y > 240) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    
    let offset_x = o.xxxx + vec4<f32>(-0.125, 0.375, 0.125, -0.375) * uniforms.texture_scale.z;
    let offset_y = o.yyyy + vec4<f32>(-0.375, -0.125, 0.375, 0.125) * uniforms.texture_scale.z;
    
    let offset_x0 = max(abs(offset_x + vec4<f32>(-1.0)) - vec4<f32>(0.5), vec4<f32>(0.0));
    let offset_x1 = max(abs(offset_x) - vec4<f32>(0.5), vec4<f32>(0.0));
    let offset_x2 = max(abs(offset_x + vec4<f32>(1.0)) - vec4<f32>(0.5), vec4<f32>(0.0));
    
    let offset_x0 = offset_x0 * offset_x0;
    let offset_x1 = offset_x1 * offset_x1;
    let offset_x2 = offset_x2 * offset_x2;
    
    let offset_yr = offset_y + vec4<f32>(-1.0);
    let offset_yr = vec4<f32>(0.02) + offset_yr * offset_yr;
    
    var acc = sample_pixel(pixel + vec2<i32>(-1, -1), offset_x0 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(0, -1), offset_x1 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(1, -1), offset_x2 + offset_yr);

    let offset_yr = vec4<f32>(0.02) + offset_y * offset_y;
    
    acc = acc + sample_pixel(pixel + vec2<i32>(-1, 0), offset_x0 + offset_yr);
    acc = acc + sample_pixel(pixel, offset_x1 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(1, 0), offset_x2 + offset_yr);

    let offset_yr = offset_y + vec4<f32>(1.0);
    let offset_yr = vec4<f32>(0.02) + offset_yr * offset_yr;
    
    acc = acc + sample_pixel(pixel + vec2<i32>(-1, 1), offset_x0 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(0, 1), offset_x1 + offset_yr);
    acc = acc + sample_pixel(pixel + vec2<i32>(1, 1), offset_x2 + offset_yr);

    return vec4<f32>(acc, 1.0);
}