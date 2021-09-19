use camera::Camera;
use hittable::{HitRecord, Hittable};
use ray::Ray;
use utility::infinity;
use vec3::{Color, Vec3};
use material::{scatter, Material};

use rand::prelude::*;

use crate::{hittable_list::HittableList, sphere::Sphere, utility::write_color, vec3::Point3};

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utility;
mod vec3;
mod material;

fn ray_color(r: &Ray, world: &HittableList, depth: i64) -> Vec3 {

    if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenutation = Vec3::default();

        if depth < 50 && scatter(&rec.material, r, &rec,&mut attenutation, &mut scattered){
            return attenutation * ray_color(&scattered, world, depth + 1);
        }
        else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } 
    else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    //image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let max_value = 255;
    let sample_count = 100;

    // World
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian{
        albedo: Vec3::new(0.1, 0.2, 0.5),
    },
    )));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian{
        albedo: Vec3::new(0.8, 0.8, 0.0),
    },
    )));

    list.push(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal{
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.3,
    },
    )));
    list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Dielectric{
        ref_idx: 1.5,
    },
    )));
    list.push(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Material::Dielectric{
        ref_idx: 1.5,
    },
    )));
    
    let world = HittableList::new(list);
    let cam = Camera::camera();
    let mut rng = rand::thread_rng();

    //Render
    print!("P3\n{} {}\n{}\n", image_width, image_height, max_value);

    for height_index in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", height_index);
        for width_index in 0..image_width {
            let mut color = Vec3::default();

            for s in 0..sample_count {
                let u = (width_index as f64 + rng.gen::<f64>()) / (image_width) as f64;
                let v = (height_index as f64 + rng.gen::<f64>()) / (image_height) as f64;

                let r = &cam.get_ray(u, v);
                color = color + ray_color(&r, &world, 0);
            }

            color = color / sample_count as f64;
            color = Vec3::new(color.x().sqrt(), color.y().sqrt(), color.z().sqrt());

            let ir = (255.99 * color.x()) as i64;
            let ig = (255.99 * color.y()) as i64;
            let ib = (255.99 * color.z()) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone");
}