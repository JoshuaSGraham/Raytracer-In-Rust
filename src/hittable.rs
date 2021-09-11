use crate::material;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Default)]
pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Material,
}

pub trait Hittable{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        None
    }
}