use bevy::{math::f32, prelude::*, reflect::List, render::render_resource::encase::private::RuntimeSizedArray};
use rand::{seq::IndexedRandom, Fill, Rng};
use rand_distr::{Distribution, LogNormal, StandardUniform};


use crate::components::*;

//const NATURAL_SELECTION: f32 = 0.7;
const MUTATION_MAX_RANGE: f32 = 0.1;
const MULT_FACTOR: f32 = 1000.;

//const UPPER_Y: f32 = 50.0;
const LOWER_Y: f32 = -340.0;
const FIRST_X: f32 = -620.0;
const GRID: (i32, i32) = (60, 40);
const CELL_SIZE: f32 = 10.0;
const X_OFFSET: f32 = 2. * CELL_SIZE;
const Y_OFFSET: f32 = 2. * CELL_SIZE - 3.;
const ODD_OFFSET: f32 = CELL_SIZE;


pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, 
            ..default()
        },
    ));
}


pub fn spawn_cells(mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<ColorMaterial>>) {

    let tissue = Tissue::default();
    let (red, green, blue) = tissue.cells[0].colour[0];

    let mut coord_x = 1.;
    for it1 in 0..GRID.0 {
        for it2 in 0..GRID.1 {
            if it2.rem_euclid(2) == 0 {
                coord_x = (FIRST_X + X_OFFSET * it1 as f32) + ODD_OFFSET;
            } else {
                coord_x = FIRST_X + X_OFFSET * it1 as f32;
            }

            let cell = Cell::default();

            commands.spawn((
                    Mesh2d(meshes.add(RegularPolygon::new(CELL_SIZE, 6))),
                    MeshMaterial2d(materials.add(Color::srgb(red, green, blue))),
                    cell,
                    Transform {
                        translation: Vec3::new(coord_x, LOWER_Y + Y_OFFSET * it2 as f32, 0.),
                        ..default()
                    }
                    )); 
        }
    }
}


pub fn cycle_cell(mut cell_query: Query<&mut Cell>,
                  time: Res<Time>) {

    for mut cell in cell_query.iter_mut() {
        cell.cell_cycle.cycle_timer.tick(time.delta());

        if cell.cell_cycle.cycle_timer.finished() {
            cell.cell_cycle.next();
            println!("{:?}", cell.cell_cycle.state);
        }
    }
}


pub fn mutate_genomic_region(
    mut commands: Commands,
    mut cell_query: Query<(Entity, &mut Cell), With<Cell>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    let mut rng = rand::rng();

    for (entity, mut cell) in cell_query.iter_mut() {
        if cell.cell_cycle.state == CellCycleState::S {
            let max_start = (cell.n_bases as f32 - cell.n_bases as f32 * 0.1).floor() as usize;

            let start: usize = rng.random_range(0..max_start);
            let range: usize = rng.random_range(1..(cell.n_bases as f32 * MUTATION_MAX_RANGE).floor() as usize);
            let mut_prob: f32 = cell.mutation_probability.dist.unwrap().sample(&mut rng);

            if mut_prob > cell.natural_selection {
                //println!("start: {}, end: {}, llr: {}", start, start+range, mut_llr);
                println!("start: {}, end: {}, llr: {}", start, start+range, mut_prob);

                let mut_type: SvState = rng.sample(StandardUniform);
                //println!("{:?}", mut_type);

                let gain_prop = (mut_prob - cell.natural_selection) * range as f32 / cell.n_bases as f32 * MULT_FACTOR;
                let (red_gain, green_gain, blue_gain) = match mut_type {
                    SvState::Amp => { (0., gain_prop, gain_prop) },
                    SvState::Del => { (gain_prop, 0., gain_prop) },
                    SvState::Inv => { (gain_prop, gain_prop, 0.) },
                    SvState::Idup => { (0., gain_prop, 0.) },
                    SvState::Ref => { (1., 1., 1.) }
                };

                // updating colours
                for it in start..start+range {
                    cell.colour[it].0 += red_gain;
                    cell.colour[it].1 += green_gain;
                    cell.colour[it].2 += blue_gain;
                }

                let mut cell_colour = (0., 0., 0.);

                for it in 0..cell.n_bases as usize {
                    cell_colour.0 += cell.colour[it].0 / cell.n_bases as f32;
                    cell_colour.1 += cell.colour[it].1 / cell.n_bases as f32;
                    cell_colour.2 += cell.colour[it].2 / cell.n_bases as f32;
                }
                println!("{:?}", cell_colour);

                cell.llr[start..start+range].fill(mut_prob);
                cell.genome[start..start+range].fill(mut_type);



                commands.entity(entity)
                    .insert(MeshMaterial2d(materials.add(Color::srgb(cell_colour.0, cell_colour.1, cell_colour.2))));
            }
        }

    }
}

/*
pub fn kill_cell(mut commands: Commands,
                  mut cell_query: Query<(Entity, &mut Cell), With<Cell>>,
                  mut materials: ResMut<Assets<ColorMaterial>>) {

    for (entity, mut cell) in cell_query.iter_mut() {
        let mut ref_flag = false;
        for pos in cell.genome.iter() {
            match pos {
                SvState::Ref => {
                    ref_flag = true;
                    break;
                },
                _ => {}
            }
        }

        if !ref_flag {
            commands.entity(entity)
                .insert(MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))));
        }
    }
}
*/
