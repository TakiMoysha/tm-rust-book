# gpui-model-preivew

A GPUI widget that shows a 3D scene: an offscreen wgpu renderer draws into a
texture, the frame is read back and painted as a GPUI image, so the viewport
behaves like any other element (drag to orbit, scroll to zoom).

- `src/scene.rs` — the scene itself: `Vec3`, triangle soup, orbit camera,
  Gouraud shading and depth testing, plus the procedural demo (torus knot,
  stage disc, grid) and a CPU rasterizer used when no GPU is available.
- `src/gpu.rs` — the wgpu backend: one pipeline, one vertex buffer per frame,
  render into `Bgra8Unorm`, read back, hand the pixels to `RenderImage`.
- `src/scene.wgsl` — vertex and fragment shaders for that pipeline.
- `src/model.rs` — FBX loading on top of [`threers`]: its loader parses the
  file, this module rebuilds the parts of the FBX transform stack the loader
  drops (`PreRotation`, the pivots, the geometric transform, FBX's Euler
  order), flattens the hierarchy into triangles and fits the model to the
  stage.
- `src/preview.rs` — the `ScenePreview` widget: GPU init and fallback, the two
  scene variants (with and without the ground), resize, orbit and stats.
- `src/main.rs` — demo window, toolbar, and the headless `--snapshot` and
  `--bench` modes.

## Running

```sh
just bin gpui-model-preview                       # assets/example.fbx by default
just bin gpui-model-preview --model path/to.fbx   # any binary or ASCII FBX
just bin gpui-model-preview --no-model            # the demo torus knot
just bin gpui-model-preview --copies 100          # 100 copies, ~600k triangles
```

`--copies N` repeats the scene as a grid of `N` copies — every triangle copied,
no instancing — and widens the stage and pulls the camera back to fit. It is a
stress test, not a feature: 100 batteries cost ~41 ms per frame on an RTX 2060
at any size, because the limit is vertex processing, not fill rate. The fog
tracks the camera distance, so a large field does not fade into the background.

Headless:

```sh
just snapshot                                     # preview.png
just bench                                        # ms per frame at three sizes
just bench --copies 100                           # the same with a field of copies
cargo run --release -- --snapshot out.png --model assets/example.fbx
```

`ZED_DEVICE_ID` in the justfile picks the discrete GPU for GPUI's compositor;
the scene renderer chooses its own adapter.
