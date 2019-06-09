use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub struct MatrixInversionError;

impl Error for MatrixInversionError{}

impl fmt::Display for MatrixInversionError
 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attempted inversion on non-invertible Matrix!")
    }
}

#[derive(Debug, PartialEq)]
pub struct PointCrossProductError
;

impl Error for PointCrossProductError{}

impl fmt::Display for PointCrossProductError
 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attempted cross product with point!")
    }
}

#[derive(Debug, PartialEq)]
pub struct SubmatrixIndexError;

impl Error for SubmatrixIndexError{}

impl fmt::Display for SubmatrixIndexError
 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attempted obtaining submatrix with row or column index out of bounds!")
    }
}