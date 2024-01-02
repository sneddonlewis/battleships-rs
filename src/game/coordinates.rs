use crate::error::AppError;

#[derive(Debug)]
pub struct Coords {
    pub row_idx: usize,
    pub col_idx: usize,
}

impl Coords {
    pub fn new(row_idx: usize, col_idx: usize) -> Self {
        Coords { row_idx, col_idx }
    }
}

impl Into<usize> for Coords {
    fn into(self) -> usize {
        10 * self.row_idx + self.col_idx
    }
}

impl TryFrom<&str> for Coords {
    type Error = AppError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // first char is a letter and represents the column from 'a' = 0 incrementally
        let letter = value
            .chars()
            .next()
            .ok_or("invalid character coordinate input")? as usize;
        let col_idx = letter - 65;
        let num_segment = &value[1..];

        // then a usize is parsed to represent the inverted row
        let inverted_row = num_segment.parse::<usize>()?;
        let row_idx = 10 - inverted_row;
        Ok(Coords { row_idx, col_idx })
    }
}
