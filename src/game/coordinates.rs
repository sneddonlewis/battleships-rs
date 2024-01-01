use crate::error::AppError;

#[derive(Debug)]
pub struct Coords {
    pub row_idx: usize,
    pub col_idx: usize,
}

impl TryFrom<&str> for Coords {
    type Error = AppError;
    fn try_from(_value: &str) -> Result<Self, Self::Error> {
        Ok(Coords {
            row_idx: 9,
            col_idx: 0,
        })
    }
}
