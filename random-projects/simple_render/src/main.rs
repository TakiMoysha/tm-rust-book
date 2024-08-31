fn main() {
    println!("Hello, world!");
}
//https://blog.danielschroeder.me/2024/05/voxel-displacement-modernizing-retro-3d/#what-ive-built
//
//
// voxel mesh - это 3d сетка, где каждый куб-ячейка заполнен или пустой.
// Кубы могут быть большими и текстурированными или маленькими и одноцветными
// яркие представители вангеры, minecraft, teardown.
//
// shall mapping - effect. 
//
// combine voxels & displacement mapping
// using low-poly meshes to model environments
// apply displacement maps to define voxel-scale surface details
// 
//
// Taking the triangle meshes that define the environment, plus limited information
// about the displacement maps, and converting them into the geometry data
// that the renderer will load onto the GPU to draw the displaced versions of the meshes4.
// This process runs on the CPU before any frames have been rendered.
// In a shipped game, you could bake the results to disk, though this isn’t too expensive;
// for the demo environment in the video, converting the entire scene on a single thread
// takes half a second.
// 
// Pre-processing the textures to generate other information needed during rendering,
// such as normal maps. This work is pretty basic, but it is slow, so you’d definitely bake the results.
// 
// Using the mesh and texture state generated above to draw the voxel-displacement geometry.
// This happens on the GPU with almost no CPU involvement per frame.
// 
// albedo map and displacement map
//
//
//
// TODO: translate
//There are limitations on how you structure the meshes and how you map the textures to them. Some of these limitations I can remove in the future, and others are unavoidable because of how the renderer works5. That said, because these are triangle meshes, not some purpose-built geometric representation, it’s possible to use many different tools to create them. I’ve used Blender to model my demo environment, but to illustrate the point, the current export format from Blender to my demo is literally just an OBJ file.
//
//
