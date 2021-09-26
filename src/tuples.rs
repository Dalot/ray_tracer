use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PV(f32, f32, f32, f32);

type Result<T> = std::result::Result<T, InvalidOperation>;

impl PV {
    pub fn new(x: f32, y: f32, z: f32, is_point: f32) -> Self {
        Self(x, y, z, is_point)
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        let margin = f32::EPSILON;
        (self.3 - 1.0).abs() < margin
    }

    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn is_valid(&self) -> bool {
        self.3 < 1.0 && self.3 > 0.0
    }

    pub fn add(&mut self, other: Self) -> Result<&mut Self> {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
        self.3 += other.3;

        if self.is_valid() {
            return Err(InvalidOperation::InvalidAddition);
        }

        Ok(self)
    }

    pub fn sub(&mut self, other: Self) -> Result<&mut Self> {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
        self.3 -= other.3;

        if self.is_valid() {
            return Err(InvalidOperation::InvalidSubtraction);
        }

        Ok(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidOperation {
    InvalidAddition,
    InvalidSubtraction,
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