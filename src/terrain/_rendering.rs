use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_graph::{self, RenderGraph, Node},
        render_resource::*,
        renderer::{RenderContext, RenderDevice},
        RenderApp, RenderStage,
        render_component::{ExtractComponent, ExtractComponentPlugin},
    },
    window::WindowDescriptor,
};
use bytemuck::{Pod, Zeroable};
use super::chunk::{ChunkData, ChunkUpdates, NODE_COUNT};
use super::tables::*;
// Extract
//as fare as I no this is how you copy data from the cpu to the gpu

#[derive(Component)]
pub struct ExtractedIsos([f32; NODE_COUNT]);

#[derive(Component)]
pub struct ExtractedPos(Vec3);
pub struct ExtractedTriTable([[i32; 16]; 256]);
pub struct ExtractedEdgeTable([u32; 256]);

pub fn extract_tables(mut commands: Commands){
    commands
        .insert_resource(ExtractedTriTable(TRI_TABLE.clone()));
    commands
        .insert_resource(ExtractedEdgeTable(EDGE_TABLE.clone()));
}

struct ExtractedChunkData(ChunkData);
pub fn extract_chunk_data(mut commands: Commands, mut chunk_data: ResMut<ChunkUpdates>){
    while let Some((isos, pos)) = chunk_data.next(){
        commands
        .spawn()
        .insert(ExtractedIsos(isos))
        .insert(Transform::from_translation(pos));
    }
}

pub fn prepare_terrain_comput(
    chunk_data: Query<(&ExtractedIsos, &Transform)>,
){
    unimplemented!()
}
pub enum TerrainRenderState{
    Loading,
    Init,
    Update,
}
pub struct ComputeTerrainNode{
    state: TerrainRenderState
}

impl Node for ComputeTerrainNode{
    fn update(&mut self, _world: &mut World) {
        unimplemented!()
    }

    fn run(
        &self, 
        graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext, 
        world: &World) -> 
        Result<(), render_graph::NodeRunError> {
        unimplemented!()
        
    }
}
