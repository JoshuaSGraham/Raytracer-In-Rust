use crate::vec3::Vec3;
use crate::ray::Ray;



pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,

}

impl Camera {
    pub fn camera(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Camera {

        let mut u = Vec3::default();
        let mut v = Vec3::default();
        let mut w = Vec3::default();


        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        w = Vec3::unit_vector(&(look_from - look_at));
        u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        v = Vec3::cross(&w, &u);

        Camera {

            origin,
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_height * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}
