//! A wgpu renderer for [`crate::scene::Scene`].
//!
//! GPUI owns its own wgpu device and does not expose it, and a window cannot
//! have a second swapchain, so this renderer draws offscreen into a texture and
//! reads the pixels back. The result is the same tightly packed BGRA buffer the
//! CPU rasterizer produced, which means the widget only swaps the
//! `Scene` -> `Frame` call.

use std::borrow::Cow;

use crate::scene::{Frame, OrbitCamera, Scene, Vec3};

/// The colour target. `Bgra8Unorm` matches GPUI's expected byte order, so the
/// readback can be handed to `RenderImage` without a per-pixel swizzle.
const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
const NEAR_PLANE: f32 = 0.05;
const FAR_PLANE: f32 = 100.0;
/// Three `vec3<f32>` attributes: position, normal, colour.
const VERTEX_STRIDE: u64 = 9 * 4;
/// Uniforms are eleven `vec4<f32>`s; see `scene.wgsl`.
const GLOBALS_FLOATS: usize = 44;

/// Uniform camera and lighting state, flattened to `f32`s for the GPU.
struct Globals {
    eye: Vec3,
    right: Vec3,
    up: Vec3,
    forward: Vec3,
    width: f32,
    height: f32,
    focal: f32,
    aspect: f32,
    key: Vec3,
    fill: Vec3,
    key_color: Vec3,
    fill_color: Vec3,
    fog_color: Vec3,
    /// Fog density, tucked into `fog_color`'s fourth component.
    fog_density: f32,
}

impl Globals {
    fn new(camera: &OrbitCamera, width: u32, height: u32) -> Self {
        let frame = camera.frame();
        Self {
            eye: frame.eye,
            right: frame.right,
            up: frame.up,
            forward: frame.forward,
            width: width as f32,
            height: height as f32,
            focal: 1.0 / (camera.fov_y * 0.5).tan(),
            aspect: width as f32 / height as f32,
            key: Vec3::new(-0.42, 0.78, 0.46).normalize(),
            fill: Vec3::new(0.66, 0.12, -0.52).normalize(),
            key_color: Vec3::new(1.00, 0.94, 0.84),
            fill_color: Vec3::new(0.36, 0.56, 1.00),
            fog_color: Vec3::new(0.012, 0.016, 0.028),
            // Fog is relative to the framing: 0.06 at the default distance,
            // and thinner when the camera is pulled back to frame a big scene.
            fog_density: 0.5 / camera.distance.max(1e-3),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut floats = [0.0_f32; GLOBALS_FLOATS];

        fn push_vec3(floats: &mut [f32; GLOBALS_FLOATS], at: usize, vector: Vec3, w: f32) {
            floats[at] = vector.x;
            floats[at + 1] = vector.y;
            floats[at + 2] = vector.z;
            floats[at + 3] = w;
        }

        push_vec3(&mut floats, 0, self.eye, 0.0);
        push_vec3(&mut floats, 4, self.right, 0.0);
        push_vec3(&mut floats, 8, self.up, 0.0);
        push_vec3(&mut floats, 12, self.forward, 0.0);
        let inv_width = 1.0 / self.width.max(1.0);
        let inv_height = 1.0 / self.height.max(1.0);
        floats[16] = self.width;
        floats[17] = self.height;
        floats[18] = inv_width;
        floats[19] = inv_height;
        floats[20] = self.focal;
        floats[21] = self.aspect;
        floats[22] = NEAR_PLANE;
        floats[23] = FAR_PLANE;
        push_vec3(&mut floats, 24, self.key, 0.0);
        push_vec3(&mut floats, 28, self.fill, 0.0);
        push_vec3(&mut floats, 32, self.key_color, 0.0);
        push_vec3(&mut floats, 36, self.fill_color, 0.0);
        push_vec3(&mut floats, 40, self.fog_color, self.fog_density);

        let mut bytes = Vec::with_capacity(GLOBALS_FLOATS * 4);
        for value in floats {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes
    }
}

/// Size-dependent render resources, recreated whenever the viewport changes.
struct Target {
    width: u32,
    height: u32,
    color: wgpu::Texture,
    color_view: wgpu::TextureView,
    depth_view: wgpu::TextureView,
    readback: wgpu::Buffer,
    padded_bytes_per_row: u32,
}

/// Offscreen wgpu renderer. Holds its own instance / adapter / device, separate
/// from the one GPUI uses for compositing.
pub struct GpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter_name: String,
    background_pipeline: wgpu::RenderPipeline,
    scene_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    globals: wgpu::Buffer,
    vertex_buffer: Option<wgpu::Buffer>,
    /// Number of vertices the current buffer can hold.
    vertex_capacity: usize,
    /// Flattened vertex data, reused between frames.
    vertex_bytes: Vec<u8>,
    /// Number of vertices in `vertex_bytes`.
    vertex_count: usize,
    pixel_scratch: Vec<u8>,
    target: Option<Target>,
}

impl GpuRenderer {
    /// Creates the GPU resources. Adapter choice honours `ZED_DEVICE_ID` (the
    /// same override GPUI uses) and otherwise prefers a discrete GPU.
    pub fn new() -> Result<Self, String> {
        pollster::block_on(Self::new_async())
    }

    async fn new_async() -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::GL,
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });

        let adapters = instance.enumerate_adapters(wgpu::Backends::all()).await;
        let adapter = select_adapter(adapters)?;
        let info = adapter.get_info();
        let adapter_name = format!("{} ({:?})", info.name, info.backend);

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("scene_preview_device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
            })
            .await
            .map_err(|error| format!("failed to create wgpu device: {error}"))?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("scene_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("scene.wgsl"))),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("scene_globals_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let globals = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("scene_globals"),
            size: (GLOBALS_FLOATS * 4) as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("scene_globals_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: globals.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("scene_pipeline_layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let color_target = wgpu::ColorTargetState {
            format: COLOR_FORMAT,
            blend: None,
            write_mask: wgpu::ColorWrites::ALL,
        };

        let background_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("background_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_background"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_background"),
                targets: &[Some(color_target.clone())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                // The background covers the whole viewport, so it must not
                // stop the scene from being drawn on top of it.
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::Always),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let scene_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("scene_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_scene"),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: VERTEX_STRIDE,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x3,
                        1 => Float32x3,
                        2 => Float32x3,
                    ],
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_scene"),
                targets: &[Some(color_target)],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                // The scene is a triangle soup whose winding is not guaranteed;
                // depth testing hides the back faces of closed shapes anyway.
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Ok(Self {
            device,
            queue,
            adapter_name,
            background_pipeline,
            scene_pipeline,
            bind_group,
            globals,
            vertex_buffer: None,
            vertex_capacity: 0,
            vertex_bytes: Vec::new(),
            vertex_count: 0,
            pixel_scratch: Vec::new(),
            target: None,
        })
    }

    /// A human-readable adapter name, e.g. `NVIDIA GeForce RTX 2060 (Vulkan)`.
    pub fn adapter_name(&self) -> &str {
        &self.adapter_name
    }

    /// Draws the scene and returns the read-back BGRA frame.
    pub fn render(
        &mut self,
        scene: &Scene,
        camera: &OrbitCamera,
        width: u32,
        height: u32,
    ) -> Frame {
        let width = width.max(1);
        let height = height.max(1);

        self.ensure_target(width, height);
        self.upload_vertices(scene);
        self.queue
            .write_buffer(&self.globals, 0, &Globals::new(camera, width, height).to_bytes());

        let target = self.target.as_ref().expect("target created above");
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("scene_encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("scene_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target.color_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &target.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            pass.set_bind_group(0, &self.bind_group, &[]);

            pass.set_pipeline(&self.background_pipeline);
            pass.draw(0..3, 0..1);

            if let Some(buffer) = &self.vertex_buffer
                && self.vertex_count > 0
            {
                pass.set_pipeline(&self.scene_pipeline);
                pass.set_vertex_buffer(0, buffer.slice(..));
                pass.draw(0..self.vertex_count as u32, 0..1);
            }
        }

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &target.color,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &target.readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(target.padded_bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(Some(encoder.finish()));

        let slice = target.readback.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("device poll failed while reading back the frame");
        receiver
            .recv()
            .expect("map_async callback never ran")
            .expect("failed to map the readback buffer");

        let mut pixels = std::mem::take(&mut self.pixel_scratch);
        pixels.clear();
        pixels.reserve((width * height * 4) as usize);
        {
            let data = slice
                .get_mapped_range()
                .expect("readback buffer is mapped");
            let row_bytes = (width * 4) as usize;
            for row in 0..height as usize {
                let start = row * target.padded_bytes_per_row as usize;
                pixels.extend_from_slice(&data[start..start + row_bytes]);
            }
        }
        target.readback.unmap();
        self.pixel_scratch = pixels;

        Frame {
            width,
            height,
            pixels: std::mem::take(&mut self.pixel_scratch),
            triangles: self.vertex_count / 3,
        }
    }

    /// Recreates the colour/depth/readback resources when the size changes.
    fn ensure_target(&mut self, width: u32, height: u32) {
        if let Some(target) = &self.target
            && target.width == width
            && target.height == height
        {
            return;
        }

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let color = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene_color"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: COLOR_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let depth = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("scene_depth"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        // `copy_texture_to_buffer` requires 256-byte aligned rows.
        let unpadded_bytes_per_row = width * 4;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

        let readback = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("scene_readback"),
            size: (padded_bytes_per_row * height) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        self.target = Some(Target {
            width,
            height,
            color_view: color.create_view(&wgpu::TextureViewDescriptor::default()),
            depth_view: depth.create_view(&wgpu::TextureViewDescriptor::default()),
            readback,
            padded_bytes_per_row,
            color,
        });
    }

    /// Flattens the scene into a vertex buffer, growing it only when needed.
    fn upload_vertices(&mut self, scene: &Scene) {
        let mut bytes = std::mem::take(&mut self.vertex_bytes);
        bytes.clear();
        bytes.reserve(scene.triangles.len() * 3 * VERTEX_STRIDE as usize);

        for triangle in &scene.triangles {
            for vertex in [triangle.a, triangle.b, triangle.c] {
                for value in [
                    vertex.position.x,
                    vertex.position.y,
                    vertex.position.z,
                    vertex.normal.x,
                    vertex.normal.y,
                    vertex.normal.z,
                    vertex.color.x,
                    vertex.color.y,
                    vertex.color.z,
                ] {
                    bytes.extend_from_slice(&value.to_le_bytes());
                }
            }
        }

        self.vertex_count = bytes.len() / VERTEX_STRIDE as usize;

        if self.vertex_count > 0 {
            if self.vertex_buffer.is_none() || self.vertex_capacity < self.vertex_count {
                let capacity = self.vertex_count.next_power_of_two().max(1024);
                self.vertex_buffer = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("scene_vertices"),
                    size: capacity as u64 * VERTEX_STRIDE,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }));
                self.vertex_capacity = capacity;
            }

            self.queue.write_buffer(
                self.vertex_buffer.as_ref().expect("created above"),
                0,
                &bytes,
            );
        }

        self.vertex_bytes = bytes;
    }
}

/// Picks the adapter to render with.
///
/// `ZED_DEVICE_ID` overrides everything (same variable and format GPUI uses),
/// then a discrete GPU wins, then whatever the instance listed first.
fn select_adapter(mut adapters: Vec<wgpu::Adapter>) -> Result<wgpu::Adapter, String> {
    if adapters.is_empty() {
        return Err("no wgpu adapters found".to_string());
    }

    let wanted = std::env::var("ZED_DEVICE_ID")
        .ok()
        .and_then(|value| parse_pci_id(&value));

    adapters.sort_by_key(|adapter| {
        let info = adapter.get_info();
        let device_match = wanted.is_some_and(|id| id == info.device);
        let discrete = info.device_type == wgpu::DeviceType::DiscreteGpu;
        let vulkan = info.backend == wgpu::Backend::Vulkan;
        (!device_match, !discrete, !vulkan)
    });

    let selected = adapters.remove(0);
    let info = selected.get_info();
    log::info!(
        "scene GPU: {} ({:?}, {:?}, device=0x{:04x})",
        info.name,
        info.backend,
        info.device_type,
        info.device
    );
    Ok(selected)
}

/// Accepts `0x1f11` or `1f11`, i.e. the same format as `ZED_DEVICE_ID`.
fn parse_pci_id(value: &str) -> Option<u32> {
    let hex = value.trim().trim_start_matches("0x").trim_start_matches("0X");
    if hex.len() == 4 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        None
    }
}
