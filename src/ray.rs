use crate::vec3::Vec3;
use crate::vec3::Point3;

pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {

    pub fn origin(&self) -> Point3{
        self.origin
    }

    pub fn direction(&self) -> Vec3{
        self.direction
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self{
        Ray {
            origin,
            direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3{
        self.origin +  self.direction * t
    }
}
