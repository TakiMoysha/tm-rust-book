/*
 * Ported from:
 * "Seascape" by Alexander Alekseev aka TDM - 2014
 * License Creative Commons Attribution-NonCommercial-ShareAlike 3.0 Unported License.
 * Contact: tdmaav@gmail.com
 */

// Uniform buffer for iTime, iResolution, iMouse
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct Uniforms {
    iTime: f32,
    iResolution: vec2f,
    iMouse: vec2f,
};

// Constants
const NUM_STEPS: i32 = 32;
const PI: f32 = 3.14159265359;
const EPSILON: f32 = 1e-3;
const EPSILON_NRM: f32 = 0.1 / uniforms.iResolution.x;

const ITER_GEOMETRY: i32 = 3;
const ITER_FRAGMENT: i32 = 5;
const SEA_HEIGHT: f32 = 0.6;
const SEA_CHOPPY: f32 = 4.0;
const SEA_SPEED: f32 = 0.8;
const SEA_FREQ: f32 = 0.16;
const SEA_BASE: vec3f = vec3f(0.0, 0.09, 0.18);
const SEA_WATER_COLOR: vec3f = vec3f(0.8, 0.9, 0.6) * 0.6;
const SEA_TIME: f32 = 1.0 + uniforms.iTime * SEA_SPEED;
const OCTAVE_M: mat2x2f = mat2x2f(1.6, 1.2, -1.2, 1.6);

// Math helpers

fn fromEuler(ang: vec3f) -> mat3x3f {
    let a1 = vec2f(sin(ang.x), cos(ang.x));
    let a2 = vec2f(sin(ang.y), cos(ang.y));
    let a3 = vec2f(sin(ang.z), cos(ang.z));

    return mat3x3f(
        vec3f(a1.y * a3.y + a1.x * a2.x * a3.x, a1.y * a2.x * a3.x + a3.y * a1.x, -a2.y * a3.x),
        vec3f(-a2.y * a1.x, a1.y * a2.y, a2.x),
        vec3f(a3.y * a1.x * a2.x + a1.y * a3.x, a1.x * a3.x - a1.y * a3.y * a2.x, a2.y * a3.y)
    );
}

fn hash(p: vec2f) -> f32 {
    let h = dot(p, vec2f(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}

fn noise(p: vec2f) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    let h00 = hash(i + vec2f(0.0, 0.0));
    let h10 = hash(i + vec2f(1.0, 0.0));
    let h01 = hash(i + vec2f(0.0, 1.0));
    let h11 = hash(i + vec2f(1.0, 1.0));

    let mx = mix(h00, h10, u.x);
    let my = mix(h01, h11, u.x);
    return -1.0 + 2.0 * mix(mx, my, u.y);
}

fn sea_octave(uv: vec2f, choppy: f32) -> f32 {
    var uv = uv + noise(uv);
    var wv = 1.0 - abs(sin(uv));
    var swv = abs(cos(uv));
    wv = mix(wv, swv, wv);
    return pow(1.0 - pow(wv.x * wv.y, 0.65), choppy);
}

fn map(p: vec3f) -> f32 {
    var freq: f32 = SEA_FREQ;
    var amp: f32 = SEA_HEIGHT;
    var choppy: f32 = SEA_CHOPPY;
    var uv: vec2f = p.xz;
    uv.x *= 0.75;

    var h: f32 = 0.0;
    for (var i: i32 = 0; i < ITER_GEOMETRY; i = i + 1) {
        var d: f32 = sea_octave((uv + SEA_TIME) * freq, choppy);
        d += sea_octave((uv - SEA_TIME) * freq, choppy);
        h += d * amp;
        uv = OCTAVE_M * uv;
        freq *= 1.9;
        amp *= 0.22;
        choppy = mix(choppy, 1.0, 0.2);
    }
    return p.y - h;
}

fn map_detailed(p: vec3f) -> f32 {
    var freq: f32 = SEA_FREQ;
    var amp: f32 = SEA_HEIGHT;
    var choppy: f32 = SEA_CHOPPY;
    var uv: vec2f = p.xz;
    uv.x *= 0.75;

    var h: f32 = 0.0;
    for (var i: i32 = 0; i < ITER_FRAGMENT; i = i + 1) {
        var d: f32 = sea_octave((uv + SEA_TIME) * freq, choppy);
        d += sea_octave((uv - SEA_TIME) * freq, choppy);
        h += d * amp;
        uv = OCTAVE_M * uv;
        freq *= 1.9;
        amp *= 0.22;
        choppy = mix(choppy, 1.0, 0.2);
    }
    return p.y - h;
}

fn diffuse(n: vec3f, l: vec3f, p: f32) -> f32 {
    return pow(dot(n, l) * 0.4 + 0.6, p);
}

fn specular(n: vec3f, l: vec3f, e: vec3f, s: f32) -> f32 {
    let nrm = (s + 8.0) / (PI * 8.0);
    let r = reflect(e, n);
    return pow(max(dot(r, l), 0.0), s) * nrm;
}

fn getSkyColor(e: vec3f) -> vec3f {
    var ey = e.y;
    ey = (max(ey, 0.0) * 0.8 + 0.2) * 0.8;
    return vec3f(pow(1.0 - ey, 2.0), 1.0 - ey, 0.6 + (1.0 - ey) * 0.4) * 1.1;
}

fn getSeaColor(p: vec3f, n: vec3f, l: vec3f, eye: vec3f, dist: vec3f) -> vec3f {
    var fresnel = clamp(1.0 - dot(n, -eye), 0.0, 1.0);
    fresnel = min(fresnel * fresnel * fresnel, 0.5);

    var reflected = getSkyColor(reflect(eye, n));
    var refracted = SEA_BASE + diffuse(n, l, 80.0) * SEA_WATER_COLOR * 0.12;

    var color = mix(refracted, reflected, fresnel);

    var atten = max(1.0 - dot(dist, dist) * 0.001, 0.0);
    color += SEA_WATER_COLOR * (p.y - SEA_HEIGHT) * 0.18 * atten;

    color += specular(n, l, eye, 600.0 * inversesqrt(dot(dist, dist)));

    return color;
}

fn getNormal(p: vec3f, eps: f32) -> vec3f {
    var n = vec3f(0.0);
    n.y = map_detailed(p);
    n.x = map_detailed(vec3f(p.x + eps, p.y, p.z)) - n.y;
    n.z = map_detailed(vec3f(p.x, p.y, p.z + eps)) - n.y;
    n.y = eps;
    return normalize(n);
}

fn heightMapTracing(ori: vec3f, dir: vec3f, out p: ptr<function, vec3f>) -> f32 {
    var tm: f32 = 0.0;
    var tx: f32 = 1000.0;
    var hx: f32 = map(ori + dir * tx);

    if (hx > 0.0) {
        *p = ori + dir * tx;
        return tx;
    }

    var hm: f32 = map(ori);

    for (var i: i32 = 0; i < NUM_STEPS; i = i + 1) {
        var tmid: f32 = mix(tm, tx, hm / (hm - hx));
        *p = ori + dir * tmid;
        var hmid: f32 = map(*p);

        if (hmid < 0.0) {
            tx = tmid;
            hx = hmid;
        } else {
            tm = tmid;
            hm = hmid;
        }

        if (abs(hmid) < EPSILON) {
            break;
        }
    }

    return mix(tm, tx, hm / (hm - hx));
}

fn getPixel(coord: vec2f, time: f32) -> vec3f {
    var uv: vec2f = coord / uniforms.iResolution.xy;
    uv = uv * 2.0 - 1.0;
    uv.x *= uniforms.iResolution.x / uniforms.iResolution.y;

    // Ray
    var ang: vec3f = vec3f(sin(time * 3.0) * 0.1, sin(time) * 0.2 + 0.3, time);
    var ori: vec3f = vec3f(0.0, 3.5, time * 5.0);
    var dir: vec3f = normalize(vec3f(uv.xy, -2.0));
    dir.z += length(uv) * 0.14;
    dir = normalize(dir) * fromEuler(ang);

    // Tracing
    var p: vec3f;
    var dist: vec3f = dir * heightMapTracing(ori, dir, &p);
    var n: vec3f = getNormal(p, dot(dist, dist) * EPSILON_NRM);
    var light: vec3f = normalize(vec3f(0.0, 1.0, 0.8));

    // Color
    var sky = getSkyColor(dir);
    var sea = getSeaColor(p, n, light, dir, dist);
    return mix(sky, sea, pow(smoothstep(0.0, -0.02, dir.y), 0.2));
}

@fragment
fn main(@builtin(position) fragCoord: vec4f) -> @location(0) vec4f {
    let time: f32 = uniforms.iTime * 0.3 + uniforms.iMouse.x * 0.01;

    var color: vec3f = vec3f(0.0);

    #ifdef AA
    for (var i: i32 = -1; i <= 1; i = i + 1) {
        for (var j: i32 = -1; j <= 1; j = j + 1) {
            let uv = fragCoord.xy + vec2f(i, j) / 3.0;
            color += getPixel(uv, time);
        }
    }
    color /= 9.0;
    #else
    color = getPixel(fragCoord.xy, time);
    #endif

    // Post-processing: gamma correction
    color = pow(color, vec3f(0.65));

    return vec4f(color, 1.0);
}
