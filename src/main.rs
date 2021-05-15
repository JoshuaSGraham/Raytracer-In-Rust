use ray::Ray;
use vec3::Vec3;

use crate::{utility::write_color, vec3::{Color, Point3}};

mod vec3;
mod utility;
mod ray;


fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64{
    let oc: Vec3 = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    if (discriminant < 0.0){
        return -1.0;
    }
    else{
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &Ray) -> Color{

    let mut t = hit_sphere(&Point3{x: 0.0, y: 0.0, z: -1.0}, 0.5, r);
    if (t > 0.0){
        let n: Vec3 = (r.at(t) - Vec3{x: 0.0, y: 0.0,z: -1.0}).unit_vector();
        return Color{x: n.x()+1.0,y: n.y()+1.0, z: n.z()+1.0} * 0.5;
    }
    let unit_direction : Vec3 = r.direction().unit_vector();
    t = 0.5 * (unit_direction.y() + 1.0); 
    return Color{x: 1.0, y: 1.0, z: 1.0} * (1.0-t) + Color{x: 0.5, y: 0.7, z: 1.0} * t;
}

fn main() {
    //image 
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio as f64) as i32;

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

            let r = Ray{origin: origin, direction: lower_left_corner + horizontal * u + vertical * v - origin};
            let pixel_color = ray_color(&r);


            //let red = width_index as f64 / (image_width -1) as f64;
            //let green = height_index as f64 / (image_height -1) as f64;
            //let blue = 0.25; 
            //let pixel_color : Color = Color{x: red, y: green, z: blue};
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}