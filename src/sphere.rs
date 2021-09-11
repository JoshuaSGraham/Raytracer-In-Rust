use crate::{hittable::HitRecord, hittable::Hittable, material::{self, Material}, ray::Ray, vec3::Point3, vec3::Vec3};

pub struct Sphere{
    pub center: Point3,
    pub radius: f64,
    material: Material,
}

impl Sphere{
    pub fn new(center: Point3, radius: f64, material: Material) -> Sphere{
        Sphere{
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self , ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b -discriminant.sqrt()) / a;

            if temp < t_max && temp > t_min {

                return Some(HitRecord{
                    t: temp,
                    p: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius,
                    material: self.material,
                });
            }

            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min{
                return Some(HitRecord{
                    t: temp,
                    p: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius,
                    material: self.material,
                });
            }
        }

        return None;
    }
}