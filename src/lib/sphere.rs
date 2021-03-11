use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Solving ||t*ray.direction - (sphere_center - ray.origin)|| = sphere_radius
        let d = ray.unit_dir();
        let s = self.center - ray.origin;
        // ||t*d - S||^2 = R^2 = (t*d - S) dot (t*d - s) = t^2*(d dot d) + t*2(d dot -S) + (S dot S)
        // Use quadratic equation:
        // let a = d.dot(d); // a = 1 because d is a unit length vector.
        // let b = 2.0 * d.dot(&-s);
        // let c = s.dot(&s) - (sphere_radius * sphere_radius);
        // let determinant = b * b - 4.0 * a * c;
        // Notice a = 1 and b is a multiple of 2:
        let half_b = d.dot(&-s);
        let c = s.len_sq() - (self.radius * self.radius);
        let determinant = half_b * half_b - c;
        if determinant < 0.0 {
            return None;
        }
        let root_determinant = determinant.sqrt();

        let t1 = -half_b - root_determinant;
        if t1 >= t_min && t1 < t_max {
            let p = ray.at(t1);
            let normal = (p - self.center).norm();
            return Some(HitRecord::new(ray, p, normal, t1));
        }

        let t2 = -half_b + root_determinant;
        if t2 >= t_min && t2 < t_max {
            let p = ray.at(t2);
            let normal = (p - self.center).norm();
            return Some(HitRecord::new(ray, p, normal, t2));
        }
        return None;
    }
}
