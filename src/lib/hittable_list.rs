use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, hittable: &'a dyn Hittable) {
        self.objects.push(hittable);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|hittable| hittable.hit(ray, t_min, t_max))
            .filter(|record| record.t.is_finite())
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
