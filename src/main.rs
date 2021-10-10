use camera::Camera;
use hittable::{HitRecord, Hittable};
use material::{scatter, Material};
use ray::Ray;
use utility::infinity;
use vec3::{Color, Vec3};

use rand::prelude::*;

use crate::{hittable_list::HittableList, sphere::Sphere, utility::write_color, vec3::Point3};

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList, depth: i64) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenutation = Vec3::default();

        if depth < 50 && scatter(&rec.material, r, &rec, &mut attenutation, &mut scattered) {
            return attenutation * ray_color(&scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    //image

    let width = 1200;
    let height = 800;
    let max_value = 255;
    let sample_count = 500;

    // World
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    let mut rng = rand::thread_rng();

    // world setup
 
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let centre = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    list.push(Box::new(Sphere::new(
                        centre,
                        0.2,
                        Material::Lambertian { albedo },
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_init(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    list.push(Box::new(Sphere::new(
                        centre,
                        0.2,
                        Material::Metal { albedo, fuzz },
                    )));
                } else {
                    list.push(Box::new(Sphere::new(
                        centre,
                        0.2,
                        Material::Dielectric { ref_idx: 1.5 },
                    )));
                }
            }
        }
    }
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric { ref_idx: 1.5 },
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    let world = HittableList::new(list);

    let aspect_ratio = width as f64 / height as f64;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let apeture = 0.1;

    let cam = Camera::camera(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        apeture,
        dist_to_focus,
    );

    //Render
    print!("P3\n{} {}\n{}\n", width, height, max_value);

    for height_index in (0..height).rev() {
        eprint!("\rScanlines remaining: {}", height_index);
        for width_index in 0..width {
            let mut color = Vec3::default();

            for s in 0..sample_count {
                let u = (width_index as f64 + rng.gen::<f64>()) / (width) as f64;
                let v = (height_index as f64 + rng.gen::<f64>()) / (height) as f64;

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
