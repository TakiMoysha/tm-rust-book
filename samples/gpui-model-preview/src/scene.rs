//! A tiny CPU renderer for a 3-D scene.
//!
//! The module is deliberately independent from GPUI: it only turns a [`Scene`]
//! and an [`OrbitCamera`] into a [`Frame`] of BGRA pixels. That makes the
//! renderer easy to test (see the snapshot binary mode in `main.rs`) and makes
//! it possible to swap it for a GPU backend later without touching the widget.

use std::f32::consts::TAU;
use std::ops::{Add, Mul, Sub};

// ---------------------------------------------------------------------------
// Math
// ---------------------------------------------------------------------------

/// A three-component vector used for positions, directions and colours.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let length = self.length();
        if length > 1e-6 { self * (1.0 / length) } else { self }
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// ---------------------------------------------------------------------------
// Camera
// ---------------------------------------------------------------------------

const NEAR_PLANE: f32 = 0.05;

/// An orbit camera: it always looks at `target` from a spherical offset.
#[derive(Clone, Copy, Debug)]
pub struct OrbitCamera {
    /// Rotation around the Y axis, in radians.
    pub yaw: f32,
    /// Elevation above the XZ plane, in radians.
    pub pitch: f32,
    pub distance: f32,
    pub target: Vec3,
    /// Vertical field of view, in radians.
    pub fov_y: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            yaw: 0.85,
            pitch: 0.42,
            distance: 8.4,
            target: Vec3::new(0.0, -0.15, 0.0),
            fov_y: 42.0_f32.to_radians(),
        }
    }
}

impl OrbitCamera {
    /// Applies a mouse drag, in radians per pixel-independent units.
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw = (self.yaw + delta_yaw).rem_euclid(TAU);
        self.pitch = (self.pitch + delta_pitch).clamp(-0.15, 1.45);
    }

    /// Zooms in or out, keeping the distance sane.
    pub fn zoom(&mut self, factor: f32) {
        // The upper bound has to clear the distance a field of copies needs.
        self.distance = (self.distance * factor).clamp(1.0, 200.0);
    }

    fn basis(&self) -> Basis {
        let cos_pitch = self.pitch.cos();
        let eye = self.target
            + Vec3::new(cos_pitch * self.yaw.cos(), self.pitch.sin(), cos_pitch * self.yaw.sin()) * self.distance;
        let forward = (self.target - eye).normalize();
        let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(forward);

        Basis {
            eye,
            right,
            up,
            forward,
        }
    }
}

/// The camera frame, cached once per frame.
struct Basis {
    eye: Vec3,
    right: Vec3,
    up: Vec3,
    forward: Vec3,
}

impl Basis {
    /// World space -> view space. In view space the camera looks down `-Z`.
    fn to_view(&self, point: Vec3) -> Vec3 {
        let offset = point - self.eye;
        Vec3::new(self.right.dot(offset), self.up.dot(offset), -self.forward.dot(offset))
    }
}

/// The camera position and orientation for the current orbit state. The GPU
/// backend needs the same basis the rasterizer uses.
pub struct CameraFrame {
    pub eye: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    /// Unit vector from `eye` towards `target`.
    pub forward: Vec3,
}

impl OrbitCamera {
    pub fn frame(&self) -> CameraFrame {
        let basis = self.basis();
        CameraFrame {
            eye: basis.eye,
            right: basis.right,
            up: basis.up,
            forward: basis.forward,
        }
    }
}

// ---------------------------------------------------------------------------
// Scene
// ---------------------------------------------------------------------------

/// One triangle vertex with everything the shader needs.
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    /// sRGB-ish base colour in the `0..1` range.
    pub color: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

/// A triangle soup. Order does not matter: back faces are culled by normal.
#[derive(Clone, Debug, Default)]
pub struct Scene {
    pub triangles: Vec<Triangle>,
}

impl Scene {
    pub fn push_triangle(&mut self, a: Vertex, b: Vertex, c: Vertex) {
        self.triangles.push(Triangle { a, b, c });
    }

    pub fn push_quad(&mut self, a: Vertex, b: Vertex, c: Vertex, d: Vertex) {
        self.push_triangle(a, b, c);
        self.push_triangle(a, c, d);
    }

    /// Repeats the soup as a grid of copies on the ground — a copy of every
    /// triangle per cell, no instancing. Returns the merged scene and the
    /// width of the grid, which is what a camera needs to frame it.
    ///
    /// The grid is square-ish and centred on the origin; the copies keep their
    /// own positions inside a cell, so a copy is never cut in half.
    pub fn repeat(&self, copies: usize, spacing: f32) -> (Scene, f32) {
        let copies = copies.max(1);
        let columns = (copies as f32).sqrt().ceil() as usize;
        let rows = copies.div_ceil(columns);

        let mut scene = Scene::default();
        scene.triangles.reserve(self.triangles.len() * copies);

        for index in 0..copies {
            let column = index % columns;
            let row = index / columns;
            let dx = (column as f32 - (columns - 1) as f32 * 0.5) * spacing;
            let dz = (row as f32 - (rows - 1) as f32 * 0.5) * spacing;

            for triangle in &self.triangles {
                scene.push_triangle(
                    shift(triangle.a, dx, dz),
                    shift(triangle.b, dx, dz),
                    shift(triangle.c, dx, dz),
                );
            }
        }

        (scene, columns.max(rows) as f32 * spacing)
    }

    /// Stretches the stage in the ground plane so the disc and the grid still
    /// cover a field of copies. Height is left alone.
    pub fn widen_stage(&mut self, factor: f32) {
        if factor <= 1.0 {
            return;
        }
        for triangle in &mut self.triangles {
            for vertex in [&mut triangle.a, &mut triangle.b, &mut triangle.c] {
                vertex.position.x *= factor;
                vertex.position.z *= factor;
            }
        }
    }
}

fn shift(vertex: Vertex, dx: f32, dz: f32) -> Vertex {
    Vertex {
        position: Vec3::new(
            vertex.position.x + dx,
            vertex.position.y,
            vertex.position.z + dz,
        ),
        ..vertex
    }
}

/// Height of the stage disc and grid. Public so a loaded model can be dropped
/// onto it.
pub const GROUND_Y: f32 = -1.55;

/// Cell size of the grid [`Scene::repeat`] builds. A model normalised by
/// [`crate::model`] is 2.6 units across, so this leaves a small gap.
pub const COPY_SPACING: f32 = 3.0;

/// Width of the default stage. A field of copies wider than this widens the
/// disc and the grid to match, so nothing stands in the void.
pub const STAGE_SPAN: f32 = 10.0;

/// Builds the demo scene: a torus knot floating above a lit ground grid.
pub fn demo_scene(with_ground: bool) -> Scene {
    demo_scene_parts(true, with_ground, with_ground)
}

/// Lets the benchmark and tests isolate the cost of each part of the scene.
pub fn demo_scene_parts(knot: bool, disc: bool, grid: bool) -> Scene {
    let mut scene = Scene::default();
    if knot {
        add_torus_knot(&mut scene);
    }
    if disc || grid {
        add_ground(&mut scene, disc, grid);
    }
    scene
}

fn add_torus_knot(scene: &mut Scene) {
    const SEGMENTS_ALONG: usize = 190;
    const SEGMENTS_AROUND: usize = 12;
    const TUBE_RADIUS: f32 = 0.34;
    const SCALE: f32 = 0.74;

    // A (2, 3) torus knot: p goes around the main ring, q winds around the tube.
    let curve = |u: f32| {
        let r = 2.0 + (3.0 * u).cos();
        Vec3::new(
            r * (2.0 * u).cos() * SCALE,
            (3.0 * u).sin() * 0.95 * SCALE,
            r * (2.0 * u).sin() * SCALE,
        )
    };

    let palette = |t: f32| {
        // A cyclic gradient, so the colour matches across the seam where the
        // last ring of the knot meets the first one.
        let cyan = Vec3::new(0.10, 0.82, 0.95);
        let violet = Vec3::new(0.58, 0.34, 0.98);
        let amber = Vec3::new(0.98, 0.62, 0.30);
        let t = t.rem_euclid(1.0) * 3.0;
        if t < 1.0 {
            cyan.lerp(violet, t)
        } else if t < 2.0 {
            violet.lerp(amber, t - 1.0)
        } else {
            amber.lerp(cyan, t - 2.0)
        }
    };

    // Pre-compute one closed ring of vertices per step along the curve.
    let mut rings: Vec<Vec<Vertex>> = Vec::with_capacity(SEGMENTS_ALONG + 1);
    for i in 0..=SEGMENTS_ALONG {
        let u = i as f32 / SEGMENTS_ALONG as f32 * TAU;
        let center = curve(u);
        let tangent = (curve(u + 0.002) - curve(u - 0.002)).normalize();

        // A stable-ish frame: avoid the tangent being parallel to the world up.
        let reference = Vec3::new(0.0, 1.0, 0.0);
        let mut normal = tangent.cross(reference);
        if normal.length() < 1e-4 {
            normal = tangent.cross(Vec3::new(1.0, 0.0, 0.0));
        }
        let normal = normal.normalize();
        let binormal = normal.cross(tangent).normalize();

        let color = palette(i as f32 / SEGMENTS_ALONG as f32);
        let mut ring = Vec::with_capacity(SEGMENTS_AROUND + 1);
        for j in 0..=SEGMENTS_AROUND {
            let v = j as f32 / SEGMENTS_AROUND as f32 * TAU;
            let offset = normal * v.cos() + binormal * v.sin();
            ring.push(Vertex {
                position: center + offset * TUBE_RADIUS,
                normal: offset,
                // Darken the inner side of the tube a little for more depth.
                color: color * (0.82 + 0.18 * v.cos().max(0.0)),
            });
        }
        rings.push(ring);
    }

    for i in 0..SEGMENTS_ALONG {
        for j in 0..SEGMENTS_AROUND {
            let a = rings[i][j];
            let b = rings[i + 1][j];
            let c = rings[i + 1][j + 1];
            let d = rings[i][j + 1];
            scene.push_quad(a, b, c, d);
        }
    }
}

fn add_ground(scene: &mut Scene, disc: bool, grid: bool) {
    const DISC_RADIUS: f32 = 5.0;
    const DISC_SEGMENTS: usize = 64;
    const DISC_RINGS: usize = 12;

    // A soft, unlit-looking pool under the model. It is made of concentric
    // rings whose colour fades into the background at the rim, so the disc has
    // no visible edge and reads as a shadow / stage.
    let stage_color = |t: f32| {
        let shadow = Vec3::new(0.006, 0.008, 0.014);
        let mid = Vec3::new(0.026, 0.036, 0.058);
        let edge = Vec3::new(0.004, 0.006, 0.011);
        if t < 0.4 {
            shadow.lerp(mid, t / 0.4)
        } else {
            mid.lerp(edge, ((t - 0.4) / 0.6).powf(0.9))
        }
    };

    let normal = Vec3::new(0.0, 1.0, 0.0);
    let vertex = |position, color| Vertex {
        position,
        normal,
        color,
    };

    if disc {
        for ring in 0..DISC_RINGS {
            let inner = DISC_RADIUS * ring as f32 / DISC_RINGS as f32;
            let outer = DISC_RADIUS * (ring + 1) as f32 / DISC_RINGS as f32;
            let inner_color = stage_color(ring as f32 / DISC_RINGS as f32);
            let outer_color = stage_color((ring + 1) as f32 / DISC_RINGS as f32);

            for segment in 0..DISC_SEGMENTS {
                let a0 = segment as f32 / DISC_SEGMENTS as f32 * TAU;
                let a1 = (segment + 1) as f32 / DISC_SEGMENTS as f32 * TAU;

                let p00 = Vec3::new(inner * a0.cos(), GROUND_Y, inner * a0.sin());
                let p01 = Vec3::new(inner * a1.cos(), GROUND_Y, inner * a1.sin());
                let p10 = Vec3::new(outer * a0.cos(), GROUND_Y, outer * a0.sin());
                let p11 = Vec3::new(outer * a1.cos(), GROUND_Y, outer * a1.sin());

                scene.push_quad(
                    vertex(p00, inner_color),
                    vertex(p10, outer_color),
                    vertex(p11, outer_color),
                    vertex(p01, inner_color),
                );
            }
        }
    }

    if grid {
        // The grid itself is made of thin quads, so it goes through the same
        // depth-tested triangle pipeline as everything else. It sits just above
        // the stage so the lines continue across it instead of disappearing.
        const GRID_Y: f32 = GROUND_Y + 0.005;
        const CELLS: i32 = 13;
        const STEP: f32 = 0.72;
        const HALF_WIDTH: f32 = 0.014;
        let extent = CELLS as f32 * STEP;

        for line in -CELLS..=CELLS {
            let offset = line as f32 * STEP;
            let major = line == 0 || line % 4 == 0;
            let color = if major {
                Vec3::new(0.14, 0.24, 0.42)
            } else {
                Vec3::new(0.075, 0.125, 0.22)
            };

            // Line parallel to the Z axis.
            add_line_quad(
                scene,
                Vec3::new(offset - HALF_WIDTH, GRID_Y, -extent),
                Vec3::new(offset + HALF_WIDTH, GRID_Y, -extent),
                Vec3::new(offset + HALF_WIDTH, GRID_Y, extent),
                Vec3::new(offset - HALF_WIDTH, GRID_Y, extent),
                color,
            );

            // Line parallel to the X axis.
            add_line_quad(
                scene,
                Vec3::new(-extent, GRID_Y, offset - HALF_WIDTH),
                Vec3::new(extent, GRID_Y, offset - HALF_WIDTH),
                Vec3::new(extent, GRID_Y, offset + HALF_WIDTH),
                Vec3::new(-extent, GRID_Y, offset + HALF_WIDTH),
                color,
            );
        }
    }
}

fn add_line_quad(scene: &mut Scene, a: Vec3, b: Vec3, c: Vec3, d: Vec3, color: Vec3) {
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let vertex = |position| Vertex {
        position,
        normal,
        color,
    };
    scene.push_quad(vertex(a), vertex(b), vertex(c), vertex(d));
}

// ---------------------------------------------------------------------------
// Rasterizer
// ---------------------------------------------------------------------------

/// A rendered frame, ready to be uploaded as a GPUI image.
pub struct Frame {
    pub width: u32,
    pub height: u32,
    /// Tightly packed BGRA bytes (the layout GPUI's sprite atlas expects).
    pub pixels: Vec<u8>,
    /// Number of triangles that survived clipping and culling.
    pub triangles: usize,
}

/// Renders `scene` from `camera` into a `width` x `height` BGRA frame.
pub fn render(scene: &Scene, camera: &OrbitCamera, width: u32, height: u32) -> Frame {
    let width = width.max(1);
    let height = height.max(1);
    let mut pixels = vec![0_u8; (width * height * 4) as usize];
    let mut depth = vec![0.0_f32; (width * height) as usize];

    // Background gradient, with a soft glow behind the model.
    for y in 0..height {
        for x in 0..width {
            let color = background(x, y, width, height);
            write_pixel(&mut pixels, width, x, y, color);
        }
    }

    let basis = camera.basis();
    let focal = 1.0 / (camera.fov_y * 0.5).tan();
    let focal_x = focal / (width as f32 / height as f32);
    // Fog is relative to the framing, not to world units: a camera pulled back
    // to see a field of copies must not turn the whole field into a silhouette.
    let fog_density = 0.5 / camera.distance.max(1e-3);

    let mut drawn = 0;
    let mut polygon: Vec<ClipVertex> = Vec::with_capacity(4);
    let mut clipped: Vec<ClipVertex> = Vec::with_capacity(4);
    let mut projected: Vec<Projected> = Vec::with_capacity(4);

    for triangle in &scene.triangles {
        let a = ClipVertex::new(triangle.a, &basis);
        let b = ClipVertex::new(triangle.b, &basis);
        let c = ClipVertex::new(triangle.c, &basis);

        // Cheap back-face cull using the average shading normal.
        let average_normal = (triangle.a.normal + triangle.b.normal + triangle.c.normal).normalize();
        let centroid = (triangle.a.position + triangle.b.position + triangle.c.position) * (1.0 / 3.0);
        if average_normal.dot(basis.eye - centroid) <= 0.0 {
            continue;
        }

        polygon.clear();
        polygon.extend_from_slice(&[a, b, c]);
        clipped.clear();
        clip_near(&polygon, &mut clipped);
        if clipped.len() < 3 {
            continue;
        }

        projected.clear();
        for vertex in &clipped {
            let view = &vertex.view;
            let w = -view.z;
            if w <= NEAR_PLANE * 0.5 {
                continue;
            }
            let inv_w = 1.0 / w;
            let ndc_x = view.x * focal_x * inv_w;
            let ndc_y = view.y * focal * inv_w;
            let world = vertex.world;
            let normal = vertex.normal;
            projected.push(Projected {
                x: (ndc_x * 0.5 + 0.5) * width as f32,
                y: (0.5 - ndc_y * 0.5) * height as f32,
                inv_w,
                // Gouraud shading: lighting is evaluated once per vertex and
                // interpolated across the triangle, which keeps the inner loop
                // free of square roots and powers.
                color: shade(vertex.color, world, normal, basis.eye, w, fog_density),
            });
        }
        if projected.len() < 3 {
            continue;
        }

        drawn += 1;
        for i in 1..projected.len() - 1 {
            rasterize_triangle(
                &projected[0],
                &projected[i],
                &projected[i + 1],
                width,
                height,
                &mut pixels,
                &mut depth,
            );
        }
    }

    Frame {
        width,
        height,
        pixels,
        triangles: drawn,
    }
}

/// A vertex after the near plane clip, still in view/world space.
#[derive(Clone, Copy)]
struct ClipVertex {
    view: Vec3,
    world: Vec3,
    normal: Vec3,
    color: Vec3,
}

impl ClipVertex {
    fn new(vertex: Vertex, basis: &Basis) -> Self {
        Self {
            view: basis.to_view(vertex.position),
            world: vertex.position,
            normal: vertex.normal,
            color: vertex.color,
        }
    }

    fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            view: self.view.lerp(other.view, t),
            world: self.world.lerp(other.world, t),
            normal: self.normal.lerp(other.normal, t),
            color: self.color.lerp(other.color, t),
        }
    }
}

fn clip_near(polygon: &[ClipVertex], out: &mut Vec<ClipVertex>) {
    for i in 0..polygon.len() {
        let current = polygon[i];
        let next = polygon[(i + 1) % polygon.len()];
        let current_distance = -current.view.z - NEAR_PLANE;
        let next_distance = -next.view.z - NEAR_PLANE;

        if current_distance >= 0.0 {
            out.push(current);
        }
        if (current_distance >= 0.0) != (next_distance >= 0.0) {
            let t = current_distance / (current_distance - next_distance);
            out.push(current.lerp(next, t));
        }
    }
}

#[derive(Clone, Copy)]
struct Projected {
    x: f32,
    y: f32,
    inv_w: f32,
    color: Vec3,
}

fn rasterize_triangle(
    p0: &Projected,
    p1: &Projected,
    p2: &Projected,
    width: u32,
    height: u32,
    pixels: &mut [u8],
    depth: &mut [f32],
) {
    let mut p = [*p0, *p1, *p2];

    let mut area = edge(p[0].x, p[0].y, p[1].x, p[1].y, p[2].x, p[2].y);
    if area.abs() < 1e-6 {
        return;
    }
    if area < 0.0 {
        p.swap(1, 2);
        area = -area;
    }

    let min_y = p[0].y.min(p[1].y).min(p[2].y).floor().max(0.0) as u32;
    let max_y = p[0].y.max(p[1].y).max(p[2].y).ceil().min(height as f32 - 1.0).max(0.0) as u32;

    if max_y < min_y {
        return;
    }

    let inv_area = 1.0 / area;

    for y in min_y..=max_y {
        let py = y as f32 + 0.5;

        // Walk only the part of the row the triangle actually covers. Grid
        // lines and near-plane-clipped geometry can have screen-space
        // bounding boxes that span the whole viewport while covering only a
        // thin sliver, so a plain bbox loop would be very wasteful.
        let mut row_min = f32::INFINITY;
        let mut row_max = f32::NEG_INFINITY;
        for i in 0..3 {
            let a = p[i];
            let b = p[(i + 1) % 3];
            let dy = b.y - a.y;
            if dy.abs() <= 1e-6 {
                continue;
            }
            if py < a.y.min(b.y) || py > a.y.max(b.y) {
                continue;
            }
            let t = (py - a.y) / dy;
            let x = a.x + (b.x - a.x) * t;
            row_min = row_min.min(x);
            row_max = row_max.max(x);
        }

        if !(row_min.is_finite() && row_max.is_finite()) || row_max < row_min {
            continue;
        }

        let start_x = (row_min - 0.5).ceil().max(0.0) as u32;
        let end_x = (row_max - 0.5).floor().min(width as f32 - 1.0).max(0.0) as u32;
        if end_x < start_x {
            continue;
        }

        for x in start_x..=end_x {
            let px = x as f32 + 0.5;

            let w0 = edge(p[1].x, p[1].y, p[2].x, p[2].y, px, py);
            let w1 = edge(p[2].x, p[2].y, p[0].x, p[0].y, px, py);
            let w2 = edge(p[0].x, p[0].y, p[1].x, p[1].y, px, py);
            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                continue;
            }

            // Screen-space barycentrics scaled by 1/w. Their sum is exactly
            // `1/z`, so one reciprocal gives perspective-correct weights.
            let a0 = w0 * inv_area * p[0].inv_w;
            let a1 = w1 * inv_area * p[1].inv_w;
            let a2 = w2 * inv_area * p[2].inv_w;
            let inv_z = a0 + a1 + a2;

            let index = (y * width + x) as usize;
            if inv_z <= depth[index] {
                continue;
            }
            depth[index] = inv_z;

            let inv_sum = 1.0 / inv_z;
            let a0 = a0 * inv_sum;
            let a1 = a1 * inv_sum;
            let a2 = a2 * inv_sum;

            let color = p[0].color * a0 + p[1].color * a1 + p[2].color * a2;

            let offset = index * 4;
            pixels[offset] = (color.z.clamp(0.0, 1.0) * 255.0) as u8;
            pixels[offset + 1] = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
            pixels[offset + 2] = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
            pixels[offset + 3] = 255;
        }
    }
}

// ---------------------------------------------------------------------------
// Shading
// ---------------------------------------------------------------------------

const KEY_DIRECTION: Vec3 = Vec3::new(-0.42, 0.78, 0.46);
const FILL_DIRECTION: Vec3 = Vec3::new(0.66, 0.12, -0.52);
const KEY_COLOR: Vec3 = Vec3::new(1.00, 0.94, 0.84);
const FILL_COLOR: Vec3 = Vec3::new(0.36, 0.56, 1.00);
/// Everything fades towards this colour in the distance.
const FOG_COLOR: Vec3 = Vec3::new(0.012, 0.016, 0.028);

/// A small studio rig: warm key light, cool fill light, specular highlight and
/// a rim light that separates silhouettes from the background.
fn shade(
    base: Vec3,
    world: Vec3,
    normal: Vec3,
    eye: Vec3,
    distance: f32,
    fog_density: f32,
) -> Vec3 {
    let n = normal.normalize();
    let view = (eye - world).normalize();
    let key = KEY_DIRECTION.normalize();
    let fill = FILL_DIRECTION.normalize();

    let diffuse_key = n.dot(key).max(0.0);
    let diffuse_fill = n.dot(fill).max(0.0);
    let half = (key + view).normalize();
    let specular = pow56(n.dot(half).max(0.0)) * 0.55;
    let rim = {
        let edge = 1.0 - n.dot(view).max(0.0);
        edge * edge * edge * 0.34
    };

    let mut color = base * (0.15 + 0.92 * diffuse_key + 0.26 * diffuse_fill)
        + KEY_COLOR * specular
        + FILL_COLOR * rim
        + base * 0.05;

    // Distant geometry sinks into the background so the grid does not end
    // abruptly at its edge.
    let fog = {
        let t = distance * fog_density;
        t / (1.0 + t)
    };
    color = color.lerp(FOG_COLOR, fog);

    // A cheap Reinhard shoulder instead of a hard clamp.
    let exposed = color * 2.0;
    Vec3::new(
        exposed.x / (1.0 + exposed.x),
        exposed.y / (1.0 + exposed.y),
        exposed.z / (1.0 + exposed.z),
    )
}

/// `x^56` without `powf`, as a specular highlight needs to stay cheap.
#[inline]
fn pow56(x: f32) -> f32 {
    let x2 = x * x;
    let x4 = x2 * x2;
    let x8 = x4 * x4;
    let x16 = x8 * x8;
    let x32 = x16 * x16;
    x32 * x16 * x8
}

#[inline]
fn edge(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (bx - ax) * (py - ay) - (by - ay) * (px - ax)
}

fn background(x: u32, y: u32, width: u32, height: u32) -> Vec3 {
    let u = x as f32 / width as f32;
    let v = y as f32 / height as f32;

    let top = Vec3::new(0.028, 0.040, 0.072);
    let bottom = Vec3::new(0.007, 0.009, 0.015);
    let mut color = top.lerp(bottom, v * (2.0 - v));

    let dx = (u - 0.5) * 1.55;
    let dy = (v - 0.42) * 1.15;
    let falloff = 1.0 + 5.0 * (dx * dx + dy * dy);
    let glow = 0.22 / (falloff * falloff);
    color = color + Vec3::new(0.10, 0.18, 0.38) * glow;

    color
}

fn write_pixel(pixels: &mut [u8], width: u32, x: u32, y: u32, color: Vec3) {
    let offset = ((y * width + x) * 4) as usize;
    pixels[offset] = (color.z.clamp(0.0, 1.0) * 255.0) as u8;
    pixels[offset + 1] = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
    pixels[offset + 2] = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
    pixels[offset + 3] = 255;
}
