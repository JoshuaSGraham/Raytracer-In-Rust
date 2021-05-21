use crate::{hittable::HitRecord, hittable::Hittable, ray::Ray, vec3::Point3, vec3::Vec3};

pub struct Sphere{
    pub center: Point3,
    pub radius: f64,
}

impl Sphere{
    pub fn new(center: Point3, radius: f64) -> Self{
        Sphere{
            center,
            radius,
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self , ray: &Ray, t_min: f64, t_max: f64, record: &HitRecord) -> bool{
        let oc: Vec3 = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius* self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(record.t);
        let outward_normal: Vec3 = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        return true;
    }
}