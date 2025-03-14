use std::{collections::HashMap, default, time::Duration};
use bevy::{prelude::*};
use rand::Rng;
use rand_distr::{Beta, BetaError};
use rand::distr::{Distribution, StandardUniform};


#[derive(Debug, Clone)]
pub enum SvState {
    Ref,
    Amp,
    Del,
    Inv,
    Idup,
}

impl Distribution<SvState> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SvState {
        let idx: u8 = rng.random_range(0..4);
        match idx {
            0 => SvState::Amp,
            1 => SvState::Del,
            2 => SvState::Inv,
            3 => SvState::Idup,
            _ => SvState::Ref,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash, PartialOrd, Ord, Clone)]
pub enum CellCycleState {
    #[default]
    G1,
    S,
    G2
}


#[derive(Component, Debug, PartialEq, Eq, Clone)]
pub struct CellCycle {
    pub state: CellCycleState,
    pub timespan: u64,
    pub cycle_timer: Timer,
    pub cycle_timespans: HashMap<CellCycleState, u64>,
}

impl CellCycle {
    pub fn start() -> Self {
        let mut cycle_timespans: HashMap<CellCycleState, u64> = HashMap::new();
        cycle_timespans.insert(CellCycleState::G1, 5);
        cycle_timespans.insert(CellCycleState::S, 3);
        cycle_timespans.insert(CellCycleState::G2, 2);

        let timespan = *cycle_timespans.get(&CellCycleState::G1).unwrap();

        CellCycle {
            state: CellCycleState::G1,
            cycle_timer: Timer::new(Duration::from_secs(timespan), TimerMode::Once),
            timespan,
            cycle_timespans

        }
    }

    pub fn next(&mut self) {
        self.state = match self.state {
            CellCycleState::G1 => CellCycleState::S,
            CellCycleState::S => CellCycleState::G2,
            CellCycleState::G2 => CellCycleState::G1
        };
        self.timespan = *self.cycle_timespans.get(&self.state).unwrap();
        self.cycle_timer.reset();
    }
}

#[derive(Debug, Clone)]
pub struct MutationDistribution {
    pub alpha: f32,
    pub beta: f32,
    pub dist: Result<Beta<f32>, BetaError>
}

impl Default for MutationDistribution {
    fn default() -> Self {
        let alpha = 1.;
        let beta = 5.;
        MutationDistribution {
            alpha,
            beta,
            dist: Beta::new(alpha, beta)
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Cell {
    pub n_bases: u64,
    pub genome: Vec<SvState>,
    pub llr: Vec<f32>,
    pub colour: Vec<(f32, f32, f32)>,
    pub cell_cycle: CellCycle,
    pub mutation_probability: MutationDistribution,
    pub natural_selection: f32,
}

impl Default for Cell {
    fn default() -> Self {
        let n_bases: u64 = 100;
        Cell{
            n_bases,
            genome: vec![SvState::Ref; n_bases as usize],
            llr: vec![0.; n_bases as usize],
            colour: vec![(0., 0., 0.); n_bases as usize],
            cell_cycle: CellCycle::start(),
            mutation_probability: MutationDistribution::default(),
            natural_selection: 0.7
        }
    }
}


#[derive(Component)]
pub struct Tissue {
    pub grid: (i32, i32),
    pub cells: Vec<Cell>
}

impl Default for Tissue {
    fn default() -> Self {
        let grid = (60, 40);
        Tissue {
            grid,
            cells: vec![Cell::default(); (grid.0 * grid.1) as usize]
        }
    }
}

impl Tissue {
    pub fn get_coords(&self, idx: usize) -> (usize, usize) {
        let it1 = idx.div_euclid(self.grid.1 as usize);
        let it2 = idx.rem_euclid(self.grid.0 as usize);
        
        return (it1, it2);
    }

    pub fn get_idx(&self, it1: usize, it2: usize) -> usize {
        return it1 * self.grid.1 as usize+ it2 - 1;
    }
}


