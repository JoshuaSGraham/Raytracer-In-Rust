use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::prelude::*;

pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
    Dielectric {},
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian { albedo: Vec3::default() }
    }
}

pub fn scatter(material: &Material, ray_in: &Ray, record: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool{
    
    
    match material {
        &Material::Lambertian { albedo } => {
            let target = record.p + record.normal + random_in_unit_sphere();
            *scattered = Ray::new(record.p, target - record.p);
            *attentuation = albedo;
            return true;
        },
        &Material::Metal { albedo } => {
            let reflected = reflect(&Vec3::unit_vector(&ray_in.direction()), &record.normal);
            *scattered = Ray::new(record.p, reflected);
            *attentuation = albedo;
            return Vec3::dot(&scattered.direction(), &record.normal) > 0.0;
        },
        &Material::Dielectric {  } => todo!(),
    } 
}

// todo: check maths on this
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
   *v - (*n * (2.0 * Vec3::dot(v, n)))
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();

    loop {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}