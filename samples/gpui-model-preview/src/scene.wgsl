// Shader for the offscreen scene renderer. It mirrors the CPU shading model
// that used to live in `scene.rs`: a warm key light, a cool fill light, a tight
// specular highlight, a rim light, exponential-ish fog and a Reinhard shoulder.

struct Globals {
    eye: vec4<f32>,
    right: vec4<f32>,
    up: vec4<f32>,
    forward: vec4<f32>,
    screen: vec4<f32>,
    // focal, aspect, near, far
    camera: vec4<f32>,
    key: vec4<f32>,
    fill: vec4<f32>,
    key_color: vec4<f32>,
    fill_color: vec4<f32>,
    fog_color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> globals: Globals;

// ---------------------------------------------------------------------------
// Background: a fullscreen triangle with the same gradient/glow as the CPU
// renderer, so the viewport still fades from blue-black at the top into almost
// black at the bottom.
// ---------------------------------------------------------------------------

@vertex
fn vs_background(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    return vec4<f32>(positions[index], 0.0, 1.0);
}

@fragment
fn fs_background(@builtin(position) fragment: vec4<f32>) -> @location(0) vec4<f32> {
    let u = fragment.x * globals.screen.z;
    let v = fragment.y * globals.screen.w;

    let top = vec3<f32>(0.028, 0.040, 0.072);
    let bottom = vec3<f32>(0.007, 0.009, 0.015);
    var color = mix(top, bottom, v * (2.0 - v));

    let dx = (u - 0.5) * 1.55;
    let dy = (v - 0.42) * 1.15;
    let falloff = 1.0 + 5.0 * (dx * dx + dy * dy);
    let glow = 0.22 / (falloff * falloff);
    color = color + vec3<f32>(0.10, 0.18, 0.38) * glow;

    return vec4<f32>(color, 1.0);
}

// ---------------------------------------------------------------------------
// Geometry
// ---------------------------------------------------------------------------

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

@vertex
fn vs_scene(input: VertexIn) -> VertexOut {
    let relative = input.position - globals.eye.xyz;
    // Camera space: +X right, +Y up, camera looks down -Z.
    let view = vec3<f32>(
        dot(globals.right.xyz, relative),
        dot(globals.up.xyz, relative),
        -dot(globals.forward.xyz, relative),
    );

    let w = -view.z;
    let focal = globals.camera.x;
    let aspect = globals.camera.y;
    let near = globals.camera.z;
    let far = globals.camera.w;

    // Maps view depth to the 0..1 depth range WebGPU uses. Dividing by the
    // perspective `w` afterwards makes this the usual non-linear depth.
    let z_clip = (-far * view.z - far * near) / (far - near);

    var out: VertexOut;
    out.clip = vec4<f32>(view.x * focal / aspect, view.y * focal, z_clip, w);
    out.world = input.position;
    out.normal = input.normal;
    out.color = input.color;
    return out;
}

@fragment
fn fs_scene(input: VertexOut) -> @location(0) vec4<f32> {
    let normal = normalize(input.normal);
    let eye = globals.eye.xyz;
    let view_dir = normalize(eye - input.world);
    let key = globals.key.xyz;
    let fill = globals.fill.xyz;

    let diffuse_key = max(dot(normal, key), 0.0);
    let diffuse_fill = max(dot(normal, fill), 0.0);
    let half_vector = normalize(key + view_dir);
    let specular = pow(max(dot(normal, half_vector), 0.0), 56.0) * 0.55;
    let edge = 1.0 - max(dot(normal, view_dir), 0.0);
    let rim = edge * edge * edge * 0.34;

    let base = input.color;
    var color = base * (0.15 + 0.92 * diffuse_key + 0.26 * diffuse_fill)
        + globals.key_color.xyz * specular
        + globals.fill_color.xyz * rim
        + base * 0.05;

    // Distant geometry sinks into the background instead of ending abruptly.
    let distance = dot(eye - input.world, globals.forward.xyz);
    // `fog_color.w` is the density the renderer derives from the camera
    // distance, so a scene framed from far away does not fade to black.
    let fog_t = distance * globals.fog_color.w;
    let fog = clamp(fog_t / (1.0 + fog_t), 0.0, 1.0);
    color = mix(color, globals.fog_color.xyz, fog);

    // A cheap Reinhard shoulder instead of a hard clamp.
    let exposed = color * 2.0;
    let toned = exposed / (vec3<f32>(1.0) + exposed);
    return vec4<f32>(toned, 1.0);
}
