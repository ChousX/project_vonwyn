use std::num::{NonZeroU32, NonZeroU64};

use bevy::{
    core_pipeline::node::MAIN_PASS_DEPENDENCIES,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{ShaderSource, ComputePipeline, BindGroup, BindGroupLayout, ShaderModuleDescriptor, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages, BindingType, StorageTextureAccess, TextureFormat, TextureViewDimension, BufferBindingType, BindGroupEntry},
        render_graph::{self, RenderGraph},
        renderer::{RenderContext, RenderDevice},
        RenderApp, RenderStage,
    },
    window::WindowDescriptor,
};
use super::chunk::{ChunkUpdateEvent, NODE_COUNT, NODE_DICTANCE, SIZE};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);

//so we need to take the iso's and run them through a comput shader 
//that will give use the mesh (Save that)
//then we need to run that through the 
pub fn gen_chunk_mesh(
    mut commands: Commands,
    mut input: EventReader<ChunkUpdateEvent>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
){
    
}


pub struct TerrainPipeline {
    marching_cubes_pipeline: ComputePipeline,
    
    texture_bind_group_layout: BindGroupLayout,
}
impl FromWorld for TerrainPipeline{
    fn from_world(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        //grabing the shader code
        let shader_source = include_str!("../../assets/shaders/marching_cubes.wgsl");
        
        let shader = render_device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("MarchingCubeShader"),
            source: ShaderSource::Wgsl(shader_source.into()),
        });

        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("isos"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { 
                            read_only: true  
                        },
                        has_dynamic_offset: true, //?
                        min_binding_size: NonZeroU64::new(32), //?
                    },
                    count: NonZeroU32::new(NODE_COUNT as u32),
                },
            ],
        });

       
        unimplemented!()
    }
}
