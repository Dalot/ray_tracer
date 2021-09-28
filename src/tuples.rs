use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
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

impl Vector {
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

    pub fn is_zero(&self) -> bool {
        let margin = f64::EPSILON;
        self.x() < margin && self.y() < margin && self.z() < margin
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
        Self {
            x: self.x() / mag,
            y: self.y() / mag,
            z: self.z() / mag,
        }
    }

    /// Dot calculates the dot product between two vectors
    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// Cross calculates the cross product between two vectors
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y() * other.z() - self.z() * other.y(),
            y: self.z() * other.x() - self.x() * other.z(),
            z: self.x() * other.y() - self.y() * other.x(),
        }
    }
}

impl ops::Add<Point> for Vector {
    type Output = Self;

    fn add(self, _rhs: Point) -> Self {
        Self {
            x: self.x() + _rhs.x(),
            y: self.y() + _rhs.y(),
            z: self.z() + _rhs.z(),
        }
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

impl ops::Add<Vector> for Vector {
    type Output = Self;

    fn add(self, _rhs: Vector) -> Self {
        Self {
            x: self.x() + _rhs.x(),
            y: self.y() + _rhs.y(),
            z: self.z() + _rhs.z(),
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, _rhs: Point) -> Vector {
        Vector {
            x: self.x() - _rhs.x(),
            y: self.y() - _rhs.y(),
            z: self.z() - _rhs.z(),
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

impl ops::Sub<Vector> for Vector {
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

impl ops::Mul<f64> for Vector {
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

impl ops::Div<f64> for Vector {
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

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(mut self) -> Self {
        self.x = -self.x();
        self.y = -self.y();
        self.z = -self.z();
        self
    }
}
