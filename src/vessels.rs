#![allow(dead_code)]

use std::fmt::Display;

use crate::error::{AppError, AppResult};

pub struct Vessel {
    class: VesselType,
    location: Vec<usize>,
    hits: Vec<usize>,
    status: VesselState,
}

impl Vessel {
    pub fn try_place(location: Vec<usize>) -> AppResult<Self> {
        let size = location.len();
        let class = size.try_into()?;
        Ok(Vessel {
            class,
            location,
            hits: Vec::with_capacity(size),
            status: VesselState::Pristine,
        })
    }

    pub fn fire(&mut self, shot: usize) -> bool {
        if self.location.contains(&shot) {
            self.hits.push(shot);
            self.update_status();
            return true;
        }
        false
    }

    fn update_status(&mut self) {
        if self.hits.len() == 0 {
            self.status = VesselState::Pristine;
            return;
        }
        if self.location.len() == self.hits.len() {
            self.status = VesselState::Sunk;
            return;
        }
        self.status = VesselState::Damaged(self.hits.len(), self.location.len())
    }
}

impl Display for Vessel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?} {:?}", self.status, self.class)
    }
}

#[derive(Debug)]
enum VesselState {
    Pristine,
    Damaged(usize, usize),
    Sunk,
}

#[derive(Debug)]
pub enum VesselType {
    Helicopter,
    Submarine,
    Cruiser,
    Battleship,
    AircraftCarrier,
}

impl TryFrom<usize> for VesselType {
    type Error = AppError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(VesselType::Helicopter),
            2 => Ok(VesselType::Submarine),
            3 => Ok(VesselType::Cruiser),
            4 => Ok(VesselType::Battleship),
            5 => Ok(VesselType::AircraftCarrier),
            _ => Err("no matching vessel for the provided size".into()),
        }
    }
}
