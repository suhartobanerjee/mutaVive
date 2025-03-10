use bevy::prelude::*;
use systems::{cycle_cell, kill_cell, mutate_genomic_region, setup_camera, spawn_cells};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_cells))
        .add_systems(Update, (mutate_genomic_region, cycle_cell, kill_cell))
        .run();
}
