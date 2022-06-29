use std::hash::Hasher;
use std::mem::MaybeUninit;
use std::collections::VecDeque;

use bevy::prelude::*;
use rand::prelude::*;
use noise::{NoiseFn, OpenSimplex, Seedable};
use super::node::{Iso, Pos, Element, TerrainNodeBundle};


pub const SIZE: (usize, usize, usize) = (32, 32, 32);
pub const NODE_COUNT: usize = SIZE.0 * SIZE.1 * SIZE.2;
pub const NODE_DICTANCE: f64 = 10.0;

#[derive(Component)]
pub struct Chunk{
    nodes: [Entity; NODE_COUNT]
}


impl Chunk{
    pub fn init(commands: &mut Commands) -> Self{
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let mut rng = thread_rng();
        let mut noise = OpenSimplex::new();
        noise.set_seed(0);
        let mut output: [MaybeUninit<Entity>; NODE_COUNT] = MaybeUninit::uninit_array();
        for i in 0..NODE_COUNT{
            output[i].write(
                commands.spawn_bundle(TerrainNodeBundle{
                    iso: Iso::gen(&mut noise, x , y , z),
                    pos: Pos([x as f32, y as f32, z as f32]),
                    element: Element::Dirt
                }).id()
            );

            x += NODE_DICTANCE;
            if x == SIZE.0 as f64 * NODE_DICTANCE{
                x = 0.0;
                y += NODE_DICTANCE;
            }
            if y == SIZE.1 as f64 * NODE_DICTANCE{
                y = 0.0;
                z += NODE_DICTANCE;
            }
        }

        let output = unsafe{
            MaybeUninit::array_assume_init(output)
        };
        Self{
            nodes: output
        }    
    }

    pub fn get_isos(&self, isos: &Query<(&Iso, &Pos, &Element)>) -> [f32; NODE_COUNT]{

        let mut output: [MaybeUninit<f32>; NODE_COUNT] = MaybeUninit::uninit_array();
        for (i, entity) in self.nodes.iter().enumerate(){
            if let Ok((iso,_,_)) = isos.get(*entity){
                output[i].write(iso.0);
            }
        }
        unsafe{
            MaybeUninit::array_assume_init(output)
        }
    }
}

pub fn test_(mut commands: Commands){
    let chunk = Chunk::init(&mut commands);
    commands.spawn().insert(chunk).insert(Update);
}


//this might be fucked
fn cubed(input: [f32; NODE_COUNT]) -> Vec<[f32; 8]>{
    let mut output = Vec::new();
    for i in 0..((SIZE.0 - 1) *(SIZE.1 -1) *(SIZE.2 -1)){
        let mut cube = [0.0 ;8];
        // 00 00
        // 00 00
        cube[0] = input[i];
        cube[1] = input[i+1];
        cube[2] = input[i+SIZE.0];
        cube[3] = input[i+SIZE.0+1];
        const TOP: usize = SIZE.0 * SIZE.1;
        cube[4] = input[i+TOP];
        cube[5] = input[i+TOP+1];
        cube[6] = input[i+TOP+SIZE.0];
        cube[7] = input[i+TOP+SIZE.0+1];

        output.push(cube);
    }

    output
}

#[derive(Component)]
pub struct Update;
pub struct ChunkUpdateEvent(Vec<[f32; 8]>);

/// Checks and removes Update Tag
/// get node values and send them off for rendering
pub fn update_chunks(
    mut commands: Commands,
    chunks: Query<(&Chunk, Entity, With<Update>)>,
    nodes: Query<(&Iso, &Pos, &Element)>,
    mut output: EventWriter<ChunkUpdateEvent>
){
    for (chunk, entity, _) in chunks.iter(){
        let isos = chunk.get_isos(&nodes);
        let cubes = cubed(isos);
        output.send(ChunkUpdateEvent(cubes));
        commands.entity(entity).remove::<Update>();
    }
}

