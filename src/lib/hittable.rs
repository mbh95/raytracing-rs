use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Vec3, outward_normal: Vec3, t: f64) -> HitRecord {
        let front_face = ray.unit_dir().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Sync {
    /// Returns a hit record describing the hit with the minimum t in the range [min_t, max_t)
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
