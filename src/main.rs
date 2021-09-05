use hittable::{HitRecord, Hittable};
use ray::Ray;
use utility::infinity;
use vec3::Vec3;

use crate::{hittable_list::HittableList, sphere::Sphere, utility::write_color, vec3::{Point3}};

mod vec3;
mod utility;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
//mod Vec3;
//mod utility;

fn ray_color(r: &Ray, world: &HittableList) -> Vec3{

    let mut rec: HitRecord = HitRecord::default();

    if world.hit(r, 0.0, infinity, &mut rec)
    {
        return Vec3::new(rec.normal().x() + 1.0, rec.normal().y() + 1.0, rec.normal().z() + 1.0) * 0.5;
    }
    else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    //image 
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio as f64) as i32;

    // World
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));
    let world = HittableList::new(list);

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3{x: 0.0,y: 0.0,z: 0.0};
    let horizontal = Vec3{x: viewport_width,y: 0.0,z: 0.0};
    let vertical = Vec3{x: 0.0,y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3{x: 0.0,y: 0.0, z: focal_length};

    //Render
    print!("P3\n{} {} \n255\n", image_width, image_height);

    for height_index in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", height_index);
        for width_index in 0..image_width {

            let u = width_index as f64 / (image_width-1) as f64;
            let v = height_index as f64 / (image_height-1) as f64;

            let r = Ray{origin: origin, direction: lower_left_corner + horizontal * u + vertical * v};
            let pixel_color = ray_color(&r, &world);


            //let red = width_index as f64 / (image_width -1) as f64;
            //let green = height_index as f64 / (image_height -1) as f64;
            //let blue = 0.25; 
            //let pixel_color : Color = Color{x: red, y: green, z: blue};
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}