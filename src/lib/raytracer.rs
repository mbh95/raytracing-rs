use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::ONE;
use rand::Rng;
use std::f64::INFINITY;

pub struct Raytracer<'a> {
    camera: &'a Camera,
    world: &'a dyn Hittable,
}

impl<'a> Raytracer<'a> {
    pub fn new(camera: &'a Camera, world: &'a dyn Hittable) -> Raytracer<'a> {
        Raytracer { camera, world }
    }

    pub fn region_color(&self, u: f64, v: f64, width: f64, height: f64, samples: u32) -> Color {
        if samples == 1 {
            return self.center_region(u, v, width, height);
        }
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _sample in 0..samples {
            pixel_color += self.sample_region(u, v, width, height);
        }
        pixel_color / (samples as f64)
    }

    pub fn center_region(&self, u: f64, v: f64, width: f64, height: f64) -> Color {
        self.uv_color(u + width / 2.0, v + height / 2.0)
    }

    pub fn sample_region(&self, u: f64, v: f64, width: f64, height: f64) -> Color {
        let mut rng = rand::thread_rng();
        let rng_u: f64 = rng.gen();
        let rng_v: f64 = rng.gen();
        let sample_du = width * rng_u;
        let sample_dv = height * rng_v;
        self.uv_color(u + sample_du, v + sample_dv)
    }

    pub fn uv_color(&self, u: f64, v: f64) -> Color {
        let ray = self.camera.get_ray(u, v);
        self.ray_color(&ray)
    }

    pub fn ray_color(&self, ray: &Ray) -> Color {
        let hit = self.world.hit(&ray, 0.0, INFINITY);
        match hit {
            Some(hit) => (hit.normal + ONE) / 2.0,
            None => {
                let t = 0.5 * (ray.unit_dir().y + 1.0);
                (1.0 - t) * ONE + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}
