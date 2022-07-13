// Data going in:
//  Read<Buffer[f32; NodeCount]>
//  Read<Buffer[Vec3; ChunkCount]>

// How to think about that data
//  I need to know what worker then convert that id to the box index
//  with that dettermin the case with the table then
//  return the mesh

//  if we want to have the vertexes indexed we need to add
//  eather a way to prun them (slow) or a vertex memo buff

use std::{borrow::Cow, num::NonZeroU32};

use crate::terrain::{
    chunk::{ChunkUpdates, NODE_COUNT},
    tables::EDGE_TABLE,
};
use bevy::{render::render_resource::{std140::*, CachedPipelineState}, prelude::Deref};
use bevy::{
    prelude::{AssetServer, Commands, FromWorld, Res},
    render::{
        render_graph::Node,
        render_resource::{
            BindGroup, BindGroupLayout, CachedComputePipelineId, ComputePipelineDescriptor,
            PipelineCache, ComputePassDescriptor
        },
        renderer::RenderDevice,
    },
};
use wgpu::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BufferSize, ShaderStages, StorageTextureAccess, BindGroupDescriptor, BindGroupEntry, BindingResource,
};

// Data out Vec<Vec3>, Vec<usize>

#[derive(Clone, AsStd140)]
pub struct EdgeTableElement {
    data: u32,
}
impl Default for EdgeTableElement {
    fn default() -> Self {
        Self { data: 0 }
    }
}

#[derive(Clone, AsStd140)]
pub struct TriTableElement {
    data: [i32; 16],
}
impl Default for TriTableElement {
    fn default() -> Self {
        Self { data: [0; 16] }
    }
}


#[derive(Clone, AsStd140)]
pub struct ChunkData {
    isos: [f32; NODE_COUNT],
    pos: Vec3,
}
impl ChunkData {
    pub fn new(isos: [f32; NODE_COUNT], pos: Vec3) -> Self {
        Self { isos, pos }
    }
}

pub struct ExtractedChunkData {
    data: Vec<ChunkData>,
}

pub fn extracte_chunk_data(mut commands: Commands, chunk_updates: Res<ChunkUpdates>) {
    let mut output = Vec::with_capacity(chunk_updates.count());
    for (entry, pos) in chunk_updates.iter() {
        output.push(ChunkData::new(*entry, pos.as_std140()))
    }
    commands.insert_resource(ExtractedChunkData { data: output });
}

#[derive(Deref)]
pub struct ChunkDataBindGroup(BindGroup);
pub fn queue_chunk_data_bind_group(
    mut commands: Commands,
    pipeline: Res<TerrainComputeMeshPipeline>,
    render_device: Res<RenderDevice>,
    chunk_data: ExtractedChunkData,
){
    let data = chunk_data.data;
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor{
        label: None,
        layout: &pipeline.chunk_data_bind,
        entries: &[BindGroupEntry{
            binding: 2,
            resource: BindingResource::Buffer(data)
        }]
    });
    commands.insert_resource(ChunkDataBindGroup(bind_group));
}
pub enum TerrainComputeState {
    Init,
    Loading,
    Update,
}

pub struct TerrainComputeMeshNode {
    state: TerrainComputeState,
}

impl Default for TerrainComputeMeshNode {
    fn default() -> Self {
        Self {
            state: TerrainComputeState::Loading
        }
    }
}

impl Node for TerrainComputeMeshNode {
    fn update(&mut self, world: &mut bevy::prelude::World) {
        let pipeline = world.resource::<TerrainComputeMeshPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        use TerrainComputeState::*;
        match self.state {
            Loading => {
                if let CachedPipelineState::Ok(_) = 
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                        self.state = Init
                }
            },
            Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = Update;
                }
            },
            _ => {}
        }
    }

    fn run(
        &self,
        graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let chunk_data_bind_group = world.resource::<ChunkDataBindGroup>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<TerrainComputeMeshPipeline>();

        let mut pass = render_context
            .command_encoder
            .begin_compute_pass(&ComputePassDescriptor::default());
        
        pass.set_bind_group(0, chunk_data_bind_group, &[]);

        use TerrainComputeState::*;
        match self.state{
            Loading => {},
            Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                //TODO: pass.dispatch(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                    pass.set_pipeline(update_pipeline);
                    //TODO: pass.dispatch(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }  
        Ok(())
    }
}

pub struct TerrainComputeMeshPipeline {
    chunk_data_bind: BindGroupLayout,
    table_bind: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for TerrainComputeMeshPipeline {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let table_bind =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: BufferSize::new(
                                    EdgeTableElement::std140_size_static() as u64,
                                ),
                            },
                            count: NonZeroU32::new(256),
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: BufferSize::new(
                                    TriTableElement::std140_size_static() as u64,
                                ),
                            },
                            count: NonZeroU32::new(256),
                        },
                    ],
                });
        let chunks = world.get_resource::<ChunkUpdates>().unwrap();
        let chunk_data_bind =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: BufferSize::new(
                                ChunkData::std140_size_static() as u64
                            ),
                        },
                        count: NonZeroU32::new(chunks.count() as u32),
                    }],
                });

        let shader = world
            .resource::<AssetServer>()
            .load(r"shaders/marching_cubes.wgsl");
        let mut pipeline_cache = world.resource_mut::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![table_bind.clone()]),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: Some(vec![table_bind.clone(), chunk_data_bind.clone()]),
            shader: shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        Self {
            table_bind,
            chunk_data_bind,
            init_pipeline,
            update_pipeline,
        }
    }
}

