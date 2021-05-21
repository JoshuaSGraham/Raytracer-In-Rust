use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;


pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord{
    pub fn set_face_normal(&self, ray: &Ray, outward_normal: &Vec3){
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if (self.front_face){
            self.normal = *outward_normal
        }
        else {
            self.normal = -*outward_normal;
        }
    }
}

pub trait Hittable{
    fn hit(&self, ray: &Ray,t_min: f64,t_max: f64,record: &HitRecord) -> bool;
}