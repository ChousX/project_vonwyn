mod chunk;
mod node;
mod rendering;
mod tables;

use bevy::{
    prelude::*,
    render::{RenderApp, RenderStage},
};

use self::{chunk::ChunkUpdates, rendering::extracte_chunk_data};

//  3D Grid of TerrainNodes -> ComputeShader -> Mesh -> VertexShader -> FragmentShader

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkUpdates::new())
            .add_system(chunk::update_chunks)
            .add_startup_system(chunk::test_);
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_system_to_stage(RenderStage::Extract, extracte_chunk_data);
    }
}
