use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit (&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //let mut temp_rec = HitRecord::default();
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in &self.list{
            if let Some(record) = object.hit(r, t_min, closest_so_far){
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}