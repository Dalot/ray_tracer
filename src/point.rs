use std::ops;
use crate::vector::Vector;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
}

impl ops::Add<Vector> for Point {
    type Output = Self;

    fn add(self, _rhs: Vector) -> Self {
        Self {
            x: self.x() + _rhs.x(),
            y: self.y() + _rhs.y(),
            z: self.z() + _rhs.z(),
        }
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, _rhs: Vector) -> Self {
        Self {
            x: self.x() - _rhs.x(),
            y: self.y() - _rhs.y(),
            z: self.z() - _rhs.z(),
        }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Self {
        Self {
            x: self.x() * _rhs,
            y: self.y() * _rhs,
            z: self.z() * _rhs,
        }
    }
}

impl ops::Div<f64> for Point {
    type Output = Self;

    fn div(self, _rhs: f64) -> Self {
        Self {
            x: self.x() / _rhs,
            y: self.y() / _rhs,
            z: self.z() / _rhs,
        }
    }
}

impl ops::Neg for Point {
    type Output = Point;

    fn neg(mut self) -> Self {
        self.x = -self.x();
        self.y = -self.y();
        self.z = -self.z();
        self
    }
}

