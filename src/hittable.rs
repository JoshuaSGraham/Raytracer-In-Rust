use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Default, Clone, Copy)]
pub struct HitRecord{
    p: Point3,
    normal: Vec3,
    t: f64,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3){
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face{
            self.normal = *outward_normal
        }
        else {
            self.normal = -*outward_normal;
        }
    }

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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}