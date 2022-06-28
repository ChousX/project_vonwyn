#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
use std::mem::MaybeUninit;

use bevy::prelude::*;

mod terrain;

fn main() {
    let mut app = App::new();
    // system stuff
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(terrain::TerrainPlugin)
        ;
    //render stuff
    app
        .run();
}
