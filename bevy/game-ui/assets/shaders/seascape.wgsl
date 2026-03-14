fn hash(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    let h00 = hash(i + vec2<f32>(0.0, 0.0));
    let h10 = hash(i + vec2<f32>(1.0, 0.0));
    let h01 = hash(i + vec2<f32>(0.0, 1.0));
    let h11 = hash(i + vec2<f32>(1.0, 1.0));

    let mx = mix(h00, h10, u.x);
    let my = mix(h01, h11, u.x);
    return -1.0 + 2.0 * mix(mx, my, u.y);
}

fn sea_octave(p: vec2<f32>, choppy: f32) -> f32 {
    var uv = p + noise(p);
    var wv = 1.0 - abs(sin(uv));
    var swv = abs(cos(uv));
    wv = mix(wv, swv, wv);
    return pow(1.0 - pow(wv.x * wv.y, 0.65), choppy);
}

const SEA_HEIGHT: f32 = 0.6;
const SEA_CHOPPY: f32 = 4.0;
const SEA_SPEED: f32 = 0.8;
const SEA_FREQ: f32 = 0.16;
const SEA_BASE: vec3<f32> = vec3<f32>(0.0, 0.09, 0.18);
const SEA_WATER_COLOR: vec3<f32> = vec3<f32>(0.8, 0.9, 0.6) * 0.6;
const OCTAVE_M: mat2x2<f32> = mat2x2<f32>(1.6, 1.2, -1.2, 1.6);

fn getSeaFromUV(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let seaTime = 1.0 + time * SEA_SPEED;
    var p = uv * 10.0;
    
    var h: f32 = 0.0;
    var freq: f32 = SEA_FREQ;
    var amp: f32 = SEA_HEIGHT;
    
    for (var i: i32 = 0; i < 4; i = i + 1) {
        var d: f32 = sea_octave((p + seaTime) * freq, SEA_CHOPPY);
        d = d + sea_octave((p - seaTime) * freq, SEA_CHOPPY);
        h = h + d * amp;
        p = OCTAVE_M * p;
        freq = freq * 1.9;
        amp = amp * 0.22;
    }
    
    let height = h * 0.5 + 0.5;
    var color = mix(SEA_BASE, SEA_WATER_COLOR + vec3<f32>(0.1, 0.1, 0.05), height);
    
    let lightDir = normalize(vec3<f32>(0.5, 0.8, 0.3));
    let normal = vec3<f32>(h * 0.5, 1.0, h * 0.5);
    let diffuse = max(dot(normalize(normal), lightDir), 0.0);
    color = color + vec3<f32>(0.3, 0.25, 0.2) * diffuse * 0.3;
    
    let fresnel = pow(1.0 - abs(uv.y * 2.0 - 1.0), 3.0);
    color = mix(color, vec3<f32>(0.6, 0.7, 0.9), fresnel * 0.3);
    
    return color;
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vertex_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(position, 1.0);
    out.uv = uv;
    out.normal = normal;
    return out;
}

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = getSeaFromUV(in.uv, 1.0);
    return vec4<f32>(color, 1.0);
}
