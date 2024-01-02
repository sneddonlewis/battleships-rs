#![allow(dead_code)]

use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub enum BoardMarker {
    FogOfWar,
    Miss,
    Hit,
    Ship,
}

impl Display for BoardMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FogOfWar => write!(f, "{}", '~'),
            Self::Miss => write!(f, "{}", 'M'),
            Self::Hit => write!(f, "{}", 'X'),
            Self::Ship => write!(f, "{}", 'O'),
        }
    }
}
