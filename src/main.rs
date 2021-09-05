use camera::Camera;
use hittable::{HitRecord, Hittable};
use ray::Ray;
use utility::infinity;
use vec3::Vec3;

use rand::prelude::*;

use crate::{hittable_list::HittableList, sphere::Sphere, utility::write_color, vec3::Point3};

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utility;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec: HitRecord = HitRecord::default();

    if world.hit(r, 0.0, f64::MAX, &mut rec) {
        return Vec3::new(
            rec.normal().x() + 1.0,
            rec.normal().y() + 1.0,
            rec.normal().z() + 1.0,
        ) * 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    //image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let max_value = 255;
    let sample_count = 100;

    // World
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);

    let cam = Camera::camera();
    let mut rng = rand::thread_rng();

    //Render
    print!("P3\n{} {}\n{}\n", image_width, image_height, max_value);

    for height_index in (0..image_height).rev() {
        //eprint!("\rScanlines remaining: {}", height_index);
        for width_index in 0..image_width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);

            for s in 0..sample_count {
                let u = (width_index as f64 + rng.gen::<f64>()) / (image_width) as f64;
                let v = (height_index as f64 + rng.gen::<f64>()) / (image_height) as f64;

                let r = &cam.get_ray(u, v);
                color = color + ray_color(&r, &world);
            }
            
            color = color / sample_count as f64;


            let ir = (255.99 * color.x()) as i64;
            let ig = (255.99 * color.y()) as i64;
            let ib = (255.99 * color.z()) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    //eprintln!("\nDone");
}