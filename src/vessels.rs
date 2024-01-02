#![allow(dead_code)]

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
            // update status
            return true;
        }
        false
    }
}

enum VesselState {
    Pristine,
    Damaged(u8, u8),
    Sunk,
}

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
