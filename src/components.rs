use std::{collections::HashMap, default, time::Duration};
use bevy::{prelude::*};
use rand_distr::{LogNormal, NormalError};


#[derive(Debug, Clone)]
pub enum SvState {
    Ref,
    Amp,
    Del,
    Inv,
    Idup,
    Complex
}

#[derive(Debug, Default, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum CellCycleState {
    #[default]
    G1,
    S,
    G2
}


#[derive(Component, Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct LogNormalDist {
    pub mean: f64,
    pub std: f64,
    pub dist: Result<LogNormal<f64>, NormalError>
}

impl LogNormalDist {
    pub fn new(mean: f64, std: f64) -> Self {
        LogNormalDist {
            mean,
            std,
            dist: LogNormal::new(mean, std)
        }
    }
}

#[derive(Component, Debug)]
pub struct Cell {
    pub genome: Vec<SvState>,
    pub llr: Vec<f64>,
    pub cell_cycle: CellCycle,
    pub mutation_rate_func: LogNormalDist,
}

impl Default for Cell {
    fn default() -> Self {
        Cell{
            genome: vec![SvState::Ref; 5],
            llr: vec![0.; 5],
            cell_cycle: CellCycle::start(),
            mutation_rate_func: LogNormalDist::new(0., 1.)
        }
    }
}

