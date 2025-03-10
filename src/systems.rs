use bevy::{prelude::*};
use rand::Rng;
use rand_distr::{LogNormal, Distribution};


use crate::components::*;

const NATURAL_SELECTION: f64 = 30.;
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

    let mut coord_x = 1.;
    for it1 in 0..GRID.0 {
        for it2 in 0..GRID.1 {
            if it2.rem_euclid(2) == 0 {
                coord_x = (FIRST_X + X_OFFSET * it1 as f32) + ODD_OFFSET;
            } else {
                coord_x = FIRST_X + X_OFFSET * it1 as f32;
            }
            commands.spawn((
                    Mesh2d(meshes.add(RegularPolygon::new(CELL_SIZE, 6))),
                    MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
                    Cell::default(),
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
            let site: usize = rng.random_range(0..cell.genome.len());
            cell.llr[site] = cell.mutation_rate_func.dist.unwrap().sample(&mut rng);

            if cell.llr[site] > NATURAL_SELECTION {
                println!("site: {}, llr: {}", site, cell.llr[site]);
                cell.genome[site] = SvState::Amp;
                commands.entity(entity)
                    .insert(MeshMaterial2d(materials.add(Color::srgb(
                                    rng.random_range(0.2..0.8),
                                    rng.random_range(0.2..0.8),
                                    rng.random_range(0.2..0.8))
                                                        )
                                          )
                           );
            }
        }

    }
}

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
                .insert(MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))));
        }
    }
}
