use crate::material;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Default)]
pub struct HitRecord{
    p: Point3,
    normal: Vec3,
    t: f64,
    pub front_face: bool,
    material: Material,
}

impl HitRecord{
    pub fn p(&self) -> Vec3{
        self.p
    }

    pub fn t(&self) -> f64{
        self.t
    }

    pub fn normal(&self) -> Vec3{
        self.normal
    }

    pub fn set_p(&mut self, val: Vec3){
        self.p = val
    }

    pub fn set_t(&mut self, val: f64){
        self.t = val
    }

    pub fn set_normal(&mut self, val: Vec3){
        self.normal = val
    }
}

pub trait Hittable{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        false
    }
}