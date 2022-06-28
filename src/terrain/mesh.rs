use bevy::prelude::*;

use super::chunk::{ChunkUpdateEvent, NODE_COUNT, NODE_DICTANCE, SIZE};

pub fn gen_chunk_mesh(
    mut commands: Commands,
    mut input: EventReader<ChunkUpdateEvent>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
){
    
}