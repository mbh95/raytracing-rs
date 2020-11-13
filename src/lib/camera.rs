use crate::ray::Ray;
use crate::vec3::{Vec3, ZERO};

pub struct Camera {
    origin: Vec3,
    center_ray_end: Vec3,
    top_right_from_center: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = ZERO;
        let center_ray_end = Vec3::new(0.0, 0.0, -focal_length);
        let top_right_from_center = Vec3::new(viewport_width / 2.0, viewport_height / 2.0, 0.0);

        Camera {
            origin,
            center_ray_end,
            top_right_from_center,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Result<Ray, String> {
        let uv = Vec3::new(u, v, 0.0);
        return Ok(Ray::new(
            self.origin,
            self.center_ray_end + uv * self.top_right_from_center - self.origin,
        )?);
    }
}
