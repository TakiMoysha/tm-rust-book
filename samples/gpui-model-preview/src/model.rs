//! FBX loading, on top of `threers`.
//!
//! [`threers::loaders::fbx::FbxLoader`] reads binary and ASCII FBX into a scene
//! graph: geometry is fan-triangulated, normals are addressed the way the file
//! declares, the hierarchy comes from `Connections`, and every mesh carries the
//! materials the file connected to it.
//!
//! The loader reads a `Model`'s local translation, rotation and scale, but not
//! the rest of the FBX transform stack — `PreRotation`, the pivots and the
//! geometric transform. Files use those for real work: the up-axis fix for a
//! Z-up mesh often lives in `PreRotation`, and detail parts are placed with
//! `GeometricTranslation`. They are read from the node tree here and folded
//! back into each node's local matrix. The Euler order differs too: FBX turns
//! a point about X first (`Rz · Ry · Rx`), while the loader's quaternion is
//! built with three.js's `XYZ` (`Rx · Ry · Rz`) — they agree on single-axis
//! rotations but not on mixed ones, so the rotation is rebuilt from the tree
//! as well.
//!
//! Nothing is rendered with threers: the model is flattened into our triangle
//! soup, so the wgpu backend and the CPU fallback keep working unchanged.

use std::{collections::HashMap, path::Path};

use threers::core::{Mesh, Object3D, ObjectArena, ObjectId, ObjectKind};
use threers::loaders::fbx::tree::Node;
use threers::loaders::fbx::{FbxLoader, FbxScene};
use threers::math::{Matrix3, Matrix4, Quaternion, Vector3};

use crate::scene::{self, Scene, Vec3, Vertex};

/// Longest edge of the model's bounding box after normalisation. The demo
/// torus knot spans roughly this much, which is what the default camera
/// distance and the ground disc are framed for.
const TARGET_EXTENT: f32 = 2.6;

/// A loaded model, flattened and fitted into our world.
pub struct Model {
    pub scene: Scene,
    /// Triangles that made it into `scene`.
    pub triangles: usize,
    /// Number of meshes in the file that had a position attribute.
    pub parts: usize,
    /// Distinct material names the file declared.
    pub materials: usize,
    pub unit_scale: f32,
    pub up_axis: u32,
}

/// Reads `path`, converts the FBX scene graph into a [`Scene`] and normalises
/// it so it stands on the ground with the same framing the demo knot had.
pub fn load(path: &Path) -> Result<Model, String> {
    let bytes = std::fs::read(path).map_err(|error| format!("cannot read: {error}"))?;
    let parsed = FbxLoader::parse_scene(&bytes).map_err(|error| error.to_string())?;
    // A second, cheap pass over the same bytes: the scene graph keeps only the
    // transform components three.js would use, and the rest is in the tree.
    let tree = FbxLoader::parse_tree(&bytes).map_err(|error| error.to_string())?;
    convert(parsed, &extra_transforms(&tree))
}

fn convert(parsed: FbxScene, extras: &HashMap<String, Extra>) -> Result<Model, String> {
    let FbxScene {
        roots,
        mut arena,
        unit_scale,
        up_axis,
        materials,
    } = parsed;

    // `apply_transform` in the loader fills in the local matrices; the world
    // matrices are only computed by this call, once per root.
    for &root in &roots {
        arena.update_world_matrices(root, Matrix4::identity());
    }

    // Worlds are recomposed here so the extra transform components reach the
    // children too, instead of stopping at the node that declared them.
    let mut worlds = HashMap::new();
    for &root in &roots {
        compose_worlds(&arena, extras, root, Matrix4::identity(), &mut worlds);
    }

    let mut meshes = Vec::new();
    for &root in &roots {
        collect_meshes(&arena, &worlds, root, &mut meshes);
    }
    if meshes.is_empty() {
        return Err("the file has no mesh geometry".to_string());
    }

    // Pass one: the bounding box in scene units, with no fit applied yet.
    let untransformed = Placement {
        unit: if unit_scale > 0.0 { unit_scale } else { 1.0 },
        up_axis,
        pivot: Vec3::new(0.0, 0.0, 0.0),
        scale: 1.0,
        lift: Vec3::new(0.0, 0.0, 0.0),
    };
    let (min, max) = bounds(&meshes, &untransformed);
    let size = max - min;
    let extent = size.x.max(size.y).max(size.z).max(1e-6);
    let scale = TARGET_EXTENT / extent;

    // Pass two: centre the model on the vertical axis, drop it onto the
    // ground and scale it to the viewport.
    let placement = Placement {
        pivot: Vec3::new((min.x + max.x) * 0.5, min.y, (min.z + max.z) * 0.5),
        scale,
        lift: Vec3::new(0.0, scene::GROUND_Y + 0.02, 0.0),
        ..untransformed
    };

    let mut scene = Scene::default();
    let mut parts = 0;
    for part in &meshes {
        let geometry = &part.mesh.geometry;
        let Some(position) = geometry.get_attribute("position") else {
            continue;
        };
        parts += 1;
        let normal = geometry.get_attribute("normal");
        let normal_matrix = Matrix3::normal_matrix(&part.world);

        for draw in part.mesh.draws() {
            let color = material_color(draw.material.color());

            // `start..end` counts index-buffer entries when the geometry is
            // indexed and vertices when it is not; FBX geometry is unindexed,
            // but threers builds indexed geometry for other formats.
            let mut corner = draw.start;
            while corner + 2 < draw.end {
                let mut triangle = [Vertex {
                    position: Vec3::new(0.0, 0.0, 0.0),
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    color,
                }; 3];

                for (slot, offset) in triangle.iter_mut().zip(0..3) {
                    let at = vertex_index(geometry, corner + offset) * 3;
                    if at + 2 >= position.array.len() {
                        break;
                    }
                    slot.position = placement.point(
                        position.array[at],
                        position.array[at + 1],
                        position.array[at + 2],
                        &part.world,
                    );
                    if let Some(attribute) = normal {
                        let stride = attribute.item_size.max(3);
                        let at = vertex_index(geometry, corner + offset) * stride;
                        if at + 2 < attribute.array.len() {
                            slot.normal = placement.normal(
                                attribute.array[at],
                                attribute.array[at + 1],
                                attribute.array[at + 2],
                                &normal_matrix,
                            );
                        }
                    }
                }

                // A degenerate stored normal would shade as a black hole; the
                // triangle's own face normal is a better guess.
                let face = (triangle[1].position - triangle[0].position)
                    .cross(triangle[2].position - triangle[0].position)
                    .normalize();
                for vertex in &mut triangle {
                    if vertex.normal.length() < 0.5 {
                        vertex.normal = face;
                    }
                }

                scene.push_triangle(triangle[0], triangle[1], triangle[2]);
                corner += 3;
            }
        }
    }

    Ok(Model {
        triangles: scene.triangles.len(),
        scene,
        parts,
        materials: materials.len(),
        unit_scale,
        up_axis,
    })
}

/// The FBX transform components the loader drops. Defaults are what an absent
/// property means, so an all-default value is a no-op.
#[derive(Clone, Copy)]
struct Extra {
    /// `Lcl Rotation`, in FBX's Euler order. Kept here rather than read from
    /// the node: the loader converts it with three.js's order, which differs.
    rotation: [f32; 3],
    rotation_offset: [f32; 3],
    rotation_pivot: [f32; 3],
    pre_rotation: [f32; 3],
    post_rotation: [f32; 3],
    scaling_offset: [f32; 3],
    scaling_pivot: [f32; 3],
    geometric_translation: [f32; 3],
    geometric_rotation: [f32; 3],
    geometric_scaling: [f32; 3],
}

impl Default for Extra {
    fn default() -> Self {
        Self {
            rotation: [0.0; 3],
            rotation_offset: [0.0; 3],
            rotation_pivot: [0.0; 3],
            pre_rotation: [0.0; 3],
            post_rotation: [0.0; 3],
            scaling_offset: [0.0; 3],
            scaling_pivot: [0.0; 3],
            geometric_translation: [0.0; 3],
            geometric_rotation: [0.0; 3],
            geometric_scaling: [1.0; 3],
        }
    }
}

impl Extra {
    fn is_identity(&self) -> bool {
        let default = Self::default();
        let same = |a: [f32; 3], b: [f32; 3]| a == b;
        same(self.rotation, default.rotation)
            && same(self.rotation_offset, default.rotation_offset)
            && same(self.rotation_pivot, default.rotation_pivot)
            && same(self.pre_rotation, default.pre_rotation)
            && same(self.post_rotation, default.post_rotation)
            && same(self.scaling_offset, default.scaling_offset)
            && same(self.scaling_pivot, default.scaling_pivot)
            && same(self.geometric_translation, default.geometric_translation)
            && same(self.geometric_rotation, default.geometric_rotation)
            && same(self.geometric_scaling, default.geometric_scaling)
    }

    /// The full FBX local matrix, in the order the FBX SDK documents:
    /// `T · Roff · Rp · Rpre · R · Rpost⁻¹ · Rp⁻¹ · Soff · Sp · S · Sp⁻¹ ·
    /// Gt · Gr · Gs`.
    fn local(&self, node: &Object3D) -> Matrix4 {
        let translation = |v: [f32; 3]| Matrix4::translation(Vector3::new(v[0], v[1], v[2]));
        let scale = |v: [f32; 3]| Matrix4::scale(Vector3::new(v[0], v[1], v[2]));

        let mut matrix = Matrix4::translation(node.position);
        matrix = matrix
            .multiply(&translation(self.rotation_offset))
            .multiply(&translation(self.rotation_pivot))
            .multiply(&euler(self.pre_rotation))
            .multiply(&euler(self.rotation))
            .multiply(&euler(self.post_rotation).invert())
            .multiply(&translation(self.rotation_pivot).invert())
            .multiply(&translation(self.scaling_offset))
            .multiply(&translation(self.scaling_pivot))
            .multiply(&Matrix4::scale(node.scale))
            .multiply(&translation(self.scaling_pivot).invert())
            .multiply(&translation(self.geometric_translation))
            .multiply(&euler(self.geometric_rotation))
            .multiply(&scale(self.geometric_scaling));
        matrix
    }
}

/// Euler angles in degrees, in FBX's `XYZ` order: the matrix is
/// `Rz · Ry · Rx`, so a point is turned about X first, then Y, then Z. (The
/// same letters in three.js mean `Rx · Ry · Rz`, which is why the rotation is
/// rebuilt here instead of read from the node's quaternion.)
fn euler(degrees: [f32; 3]) -> Matrix4 {
    let axis = |index: usize| {
        let mut angles = [0.0; 3];
        angles[index] = degrees[index].to_radians();
        Matrix4::from_quaternion(Quaternion::from_euler_xyz(angles[0], angles[1], angles[2]))
    };
    axis(2).multiply(&axis(1)).multiply(&axis(0))
}

fn extra_transforms(tree: &Node) -> HashMap<String, Extra> {
    let mut extras = HashMap::new();
    let Some(objects) = tree.child("Objects") else {
        return extras;
    };

    for model in objects.children_named("Model") {
        let extra = Extra {
            rotation: model.property70_vec3("Lcl Rotation").unwrap_or([0.0; 3]),
            rotation_offset: model.property70_vec3("RotationOffset").unwrap_or([0.0; 3]),
            rotation_pivot: model.property70_vec3("RotationPivot").unwrap_or([0.0; 3]),
            pre_rotation: model.property70_vec3("PreRotation").unwrap_or([0.0; 3]),
            post_rotation: model.property70_vec3("PostRotation").unwrap_or([0.0; 3]),
            scaling_offset: model.property70_vec3("ScalingOffset").unwrap_or([0.0; 3]),
            scaling_pivot: model.property70_vec3("ScalingPivot").unwrap_or([0.0; 3]),
            geometric_translation: model
                .property70_vec3("GeometricTranslation")
                .unwrap_or([0.0; 3]),
            geometric_rotation: model
                .property70_vec3("GeometricRotation")
                .unwrap_or([0.0; 3]),
            geometric_scaling: model
                .property70_vec3("GeometricScaling")
                .unwrap_or([1.0; 3]),
        };
        if !extra.is_identity() {
            extras.insert(node_name(model), extra);
        }
    }

    extras
}

/// `threers` cleans an object name the same way: the first string property, cut
/// at the NUL that separates it from the class name, and any `::` namespace
/// dropped.
fn node_name(node: &Node) -> String {
    let raw = node
        .properties
        .iter()
        .find_map(|property| property.as_str())
        .unwrap_or_default();
    let cleaned = raw.split('\0').next().unwrap_or(raw);
    match cleaned.rsplit_once("::") {
        Some((_, name)) => name.to_string(),
        None => cleaned.to_string(),
    }
}

fn compose_worlds(
    arena: &ObjectArena,
    extras: &HashMap<String, Extra>,
    id: ObjectId,
    parent: Matrix4,
    out: &mut HashMap<ObjectId, Matrix4>,
) {
    let Some(node) = arena.get(id) else {
        return;
    };
    let local = match extras.get(&node.name) {
        Some(extra) => extra.local(node),
        None => node.matrix,
    };
    let world = parent.multiply(&local);
    out.insert(id, world);

    for &child in &node.children {
        compose_worlds(arena, extras, child, world, out);
    }
}

/// A mesh and the world matrix it hangs under.
struct Part<'a> {
    world: Matrix4,
    mesh: &'a Mesh,
}

/// Every mesh reachable from `id`, borrowing its geometry from the arena.
fn collect_meshes<'a>(
    arena: &'a ObjectArena,
    worlds: &HashMap<ObjectId, Matrix4>,
    id: ObjectId,
    out: &mut Vec<Part<'a>>,
) {
    let Some(node) = arena.get(id) else {
        return;
    };
    if let ObjectKind::Mesh(mesh) = &node.kind {
        out.push(Part {
            world: worlds.get(&id).copied().unwrap_or(node.matrix_world),
            mesh,
        });
    }
    for &child in &node.children {
        collect_meshes(arena, worlds, child, out);
    }
}

/// How file coordinates become scene coordinates: the FBX unit and up axis,
/// plus the fit that pass two computes.
#[derive(Clone, Copy)]
struct Placement {
    unit: f32,
    up_axis: u32,
    /// The point that becomes the world origin.
    pivot: Vec3,
    scale: f32,
    /// Where the pivot lands after scaling.
    lift: Vec3,
}

impl Placement {
    fn point(&self, x: f32, y: f32, z: f32, world: &Matrix4) -> Vec3 {
        let m = &world.elements;
        let world_position = Vec3::new(
            m[0] * x + m[4] * y + m[8] * z + m[12],
            m[1] * x + m[5] * y + m[9] * z + m[13],
            m[2] * x + m[6] * y + m[10] * z + m[14],
        ) * self.unit;

        (self.axis(world_position) - self.pivot) * self.scale + self.lift
    }

    fn normal(&self, x: f32, y: f32, z: f32, normal_matrix: &Matrix3) -> Vec3 {
        let m = &normal_matrix.elements;
        let normal = Vec3::new(
            m[0] * x + m[3] * y + m[6] * z,
            m[1] * x + m[4] * y + m[7] * z,
            m[2] * x + m[5] * y + m[8] * z,
        );
        self.axis(normal).normalize()
    }

    /// Files may be Z-up; the scene is always Y-up.
    fn axis(&self, vector: Vec3) -> Vec3 {
        if self.up_axis == 2 {
            Vec3::new(vector.x, vector.z, -vector.y)
        } else {
            vector
        }
    }
}

fn bounds(meshes: &[Part<'_>], placement: &Placement) -> (Vec3, Vec3) {
    let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

    for part in meshes {
        let Some(position) = part.mesh.geometry.get_attribute("position") else {
            continue;
        };
        for point in position.array.chunks_exact(3) {
            let p = placement.point(point[0], point[1], point[2], &part.world);
            min = Vec3::new(min.x.min(p.x), min.y.min(p.y), min.z.min(p.z));
            max = Vec3::new(max.x.max(p.x), max.y.max(p.y), max.z.max(p.z));
        }
    }

    (min, max)
}

fn vertex_index(geometry: &threers::core::BufferGeometry, corner: u32) -> usize {
    match &geometry.index {
        Some(indices) => indices.get(corner as usize).copied().unwrap_or(0) as usize,
        None => corner as usize,
    }
}

fn material_color(color: threers::math::Color) -> Vec3 {
    let rgb = Vec3::new(color.r, color.g, color.b);
    if rgb.x.max(rgb.y).max(rgb.z) < 0.02 {
        // FBX files without a `DiffuseColor` read as black, which would render
        // as a silhouette rather than a part.
        Vec3::new(0.58, 0.60, 0.63)
    } else {
        rgb
    }
}
