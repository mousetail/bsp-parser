use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub const AXISES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn get_axis(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn distance_squared(&self, other: &Self) -> f32 {
        let diff = self - other;
        return diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
    }

    pub fn dot(&self, other: &Self) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn normalize(&self) -> Self {
        let distance = self
            .distance_squared(&Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            })
            .sqrt();
        return Vec3 {
            x: self.x / distance,
            y: self.y / distance,
            z: self.z / distance,
        };
    }

    pub fn set_axis(&self, axis: Axis, value: f32) -> Vec3 {
        match axis {
            Axis::X => Vec3 {
                x: value,
                y: self.y,
                z: self.z,
            },
            Axis::Y => Vec3 {
                x: self.x,
                y: value,
                z: self.z,
            },
            Axis::Z => Vec3 {
                x: self.x,
                y: self.y,
                z: value,
            },
        }
    }

    pub fn reflect(&self, axis: Axis) -> Vec3 {
        self.set_axis(axis, -self.get_axis(axis))
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
