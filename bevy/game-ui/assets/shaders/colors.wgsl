struct ParamsInput {
    material_time_offset: f32,
    material_color: vec4<f32>,
}
@group(0) @binding(0) var<uniform> params: ParamsInput;
@group(0) @binding(2) var material_color_texture: texture_2d<f32>;
@group(0) @binding(3) var material_color_sampler: sampler;

const COLOR_MULTIPLIER = vec4<f32>(1.0, 1.0, 1.0, 1.0);

@fragment
fn fragment_main(
    @builtin(position) frag_coord: vec4<f32>,
) -> @location(0) vec4<f32> {
    let t = params.material_time_offset;

    let red = 0.5 + 0.5 * sin(t * 2.0);
    let blue = 0.5 + 0.5 * sin(t * 2.0 + 1.57);

    return params.material_color * textureSample(material_color_texture, material_color_sampler, frag_coord.xy) * COLOR_MULTIPLIER * t;
}
