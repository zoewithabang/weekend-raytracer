use crate::point3::Point3;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.direction * t)
    }

    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn new() -> Self {
        Self {
            origin: Point3::new(),
            direction: Vec3::new()
        }
    }
}