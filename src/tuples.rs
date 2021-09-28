use core::fmt;
use std::{
    fmt::{Display, Formatter},
    ops,
};

use crate::helpers::f64_equal;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PV(f64, f64, f64, f64);

type Result<T> = std::result::Result<T, InvalidOperation>;

impl PV {
    pub fn new(x: f64, y: f64, z: f64, is_point: f64) -> Self {
        Self(x, y, z, is_point)
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        f64_equal(self.3, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        !self.is_point()
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn is_valid(&self) -> bool {
        self.3 < 1.0 && self.3 > 0.0
    }

    pub fn is_zero(&self) -> bool {
        let margin = f64::EPSILON;
        self.0 < margin && self.1 < margin && self.2 < margin
    }

    /// Mag calculated the magnitude of a vector
    pub fn mag(&self) -> f64 {
        // calculate the first 2D hypotenuse which will be the adjacent side for the 3D hypotenuse
        let adj = self.x().powi(2) + self.z().powi(2);
        (adj + self.y().powi(2)).sqrt().abs()
    }

    /// Norm normalizes a vector
    pub fn norm(&self) -> Self {
        let mag = self.mag();
        Self(self.x() / mag, self.y() / mag, self.z() / mag, 0.0)
    }

    /// Dot calculates the dot product between two vectors
    pub fn dot(&self, other: &Self) -> Result<f64> {
        if self.is_point() {
            return Err(InvalidOperation::InvalidDotProduct);
        }

        Ok(self.x() * other.x() + self.y() * other.y() + self.z() * other.z())
    }

    /// Cross calculates the cross product between two vectors
    pub fn cross(&self, other: &Self) -> Result<PV> {
        if self.is_point() {
            return Err(InvalidOperation::InvalidCrossProduct);
        }

        Ok(
            Self(
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(), 
                0.0
            )
        )
    }
}

impl ops::Add<PV> for PV {
    type Output = Result<Self>;

    fn add(self, _rhs: PV) -> Result<Self> {
        let res = Self(
            self.0 + _rhs.0,
            self.1 + _rhs.1,
            self.2 + _rhs.2,
            self.3 + _rhs.3,
        );

        if self.is_valid() {
            return Err(InvalidOperation::InvalidAddition);
        }

        Ok(res)
    }
}

impl ops::Sub<PV> for PV {
    type Output = Result<Self>;

    fn sub(self, _rhs: PV) -> Result<Self> {
        let res = Self(
            self.0 - _rhs.0,
            self.1 - _rhs.1,
            self.2 - _rhs.2,
            self.3 - _rhs.3,
        );

        if self.is_valid() {
            return Err(InvalidOperation::InvalidSubtraction);
        }

        Ok(res)
    }
}

impl ops::Mul<f64> for PV {
    type Output = Result<Self>;

    fn mul(self, _rhs: f64) -> Result<Self> {
        let res = Self(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs, self.3 * _rhs);

        if self.is_valid() {
            return Err(InvalidOperation::InvalidMultiplication);
        }

        Ok(res)
    }
}

impl ops::Div<f64> for PV {
    type Output = Result<Self>;

    fn div(self, _rhs: f64) -> Result<Self> {
        let margin = f64::EPSILON;
        if (self.3 - 1.0).abs() < margin {
            return Err(InvalidOperation::InvalidDivision);
        }

        let res = Self(self.0 / _rhs, self.1 / _rhs, self.2 / _rhs, self.3 / _rhs);

        if self.is_valid() {
            return Err(InvalidOperation::InvalidDivision);
        }

        Ok(res)
    }
}

impl ops::Neg for PV {
    type Output = PV;

    fn neg(mut self) -> PV {
        self.0 = -self.0;
        self.1 = -self.1;
        self.2 = -self.2;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidOperation {
    InvalidAddition,
    InvalidSubtraction,
    InvalidMultiplication,
    InvalidDivision,
    InvalidDotProduct,
    InvalidCrossProduct
}

#[derive(Debug, Clone)]
pub struct InvalidAddition;

impl Display for InvalidAddition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid addtion. Are you trying to add a point to a point?"
        )
    }
}

#[derive(Debug, Clone)]
pub struct InvalidSubtraction;

impl Display for InvalidSubtraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid subtraction. Are you trying to subtract a point from a vector?"
        )
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMultiplication;

impl Display for InvalidMultiplication {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid multiplication.")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidDivision;

impl Display for InvalidDivision {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid division.")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidDotProduct;

impl Display for InvalidDotProduct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Invalid dot product. Are you trying to make a dot product with a point?")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidCrossProduct;

impl Display for InvalidCrossProduct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid cross product. Are you trying to calculate the cross product with a point?"
        )
    }
}