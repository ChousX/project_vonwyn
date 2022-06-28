mod node;
mod chunk;
mod mesh;

use bevy::prelude::*;

//  3D Grid of TerrainNodes -> ComputeShader -> Mesh -> VertexShader -> FragmentShader

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_event::<chunk::ChunkUpdateEvent>()
            .add_system(chunk::update_chunks)
            .add_startup_system(chunk::test_)
            ;
    }
}
