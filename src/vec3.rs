use std::ops::{Index, IndexMut, Neg, Add, Mul, Div, AddAssign, DivAssign, MulAssign, Sub};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::from(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() - rhs.x()
        )
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Self {
            e: [x, y, z]
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn new() -> Self {
        Self {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z()
            ]
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z()
            ]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self.x() + rhs.x(),
                self.y() + rhs.y(),
                self.z() + rhs.z()
            ]
        }
    }
}

impl Display for &Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{0} {1} {2}", self.x(), self.y(), self.z())
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs
            ]
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs
            ]
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [
                self.x() * rhs,
                self.y() * rhs,
                self.z() * rhs
            ]
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            e: [-self.x(), -self.y(), -self.z()]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z()
            ]
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.x() - rhs.x(),
                self.y() - rhs.y(),
                self.z() - rhs.z()
            ]
        }
    }
}