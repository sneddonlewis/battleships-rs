#![allow(dead_code)]

pub enum VesselType {
    Helicopter,
}

impl VesselType {
    fn size(&self) -> usize {
        match self {
            VesselType::Helicopter => 1,
        }
    }
}
