use crate::vec3::Vec3;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Vec3 {
        Vec3::new(self[0] + other, self[1] + other, self[2] + other)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        Vec3::new(self[0] - other, self[1] - other, self[2] - other)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self[0] * other, self[1] * other, self[2] * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other[0], self * other[1], self * other[2])
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] / other[0], self[1] / other[1], self[2] / other[2])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self[0] / other, self[1] / other, self[2] / other)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self / other[0], self / other[1], self / other[2])
    }
}
