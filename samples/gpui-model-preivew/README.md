# gpui-model-preivew

- `src/scene.rs` — pure renderer: camera, procedural geometry, z-buffer,
  Gouraud shading, BGRA output. No GPUI dependency, easy to snapshot and test.
- `src/preview.rs` — the `ScenePreview` widget: `canvas` prepaint rasterizes,
  paint uploads the image; `Window`-level mouse listeners handle orbiting.
- `src/main.rs` — demo window, toolbar, snapshot and benchmark modes.
