use crate::{hittable::HitRecord, hittable::Hittable, ray::Ray, vec3::Point3, vec3::Vec3};

pub struct Sphere{
    pub center: Point3,
    pub radius: f64,
}

impl Sphere{
    pub fn new(center: Point3, radius: f64) -> Sphere{
        Sphere{
            center,
            radius,
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self , ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool{
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b -discriminant.sqrt()) / a;

            if temp < t_max && temp > t_min {
                record.set_t(temp);
                record.set_p(ray.point_at(record.t()));
                record.set_normal((record.p() - self.center) / self.radius);
                return true;
            }

            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min{
                record.set_t(temp);
                record.set_p(ray.point_at(record.t()));
                record.set_normal((record.p() - self.center) / self.radius);
                return true;
            }
        }

        return false;
    }
}