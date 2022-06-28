use bevy::prelude::*;
use bevy::ecs::bundle::Bundle;
use noise::OpenSimplex;

#[derive(Bundle)]
pub struct TerrainNodeBundle{
    pub iso: Iso,
    pub pos: Pos,
    pub element: Element,
}

#[derive(Component)]
pub struct Pos(pub [f32; 3]);

#[derive(Component)]
pub struct Iso(pub f32);

impl Iso{
    pub fn gen(noise: &mut OpenSimplex, x: f64, y: f64, z: f64) -> Self{
        Self(0.0)
    }
}



#[derive(Component)]
pub enum Element{
    Air,
    Dirt
}
