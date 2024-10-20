use core::fmt;

pub enum IntoMatrixError {
    // zero length vector given
    ZeroLengthGivenToConvert,
    // vectors of verying lengths
    
}

#[derive(fmt::Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl TryFrom<Vec<f64>> for Matrix {
    type Error = IntoMatrixError;
    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(IntoMatrixError::ZeroLengthGivenToConvert);
        };
        Ok(
            Matrix {
            rows: value.len(),
            cols: 1,
            data: vec!(value),
            }
        )
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(value: Vec<Vec<f64>>) -> Self {
        let lens = value.clone().into_iter().map(|v| {v.len()});

        Matrix {
            rows: value[0].len(),
            cols: value.len(),
            data: value,
        }
    }
}

impl From<f64> for Matrix {
    fn from(value: f64) -> Self {
        Matrix {
            rows: 1,
            cols: 1,
            data: vec!(vec!(value)),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_from_f64_for_matrix() {
        assert_eq!(Matrix { rows: 1, cols: 1, data: vec!(vec!(3.4)) }, Matrix::from(3.4))
    }
}