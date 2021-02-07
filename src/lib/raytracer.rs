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
    pub fn region_color(
        &mut self,
        u: f64,
        v: f64,
        width: f64,
        height: f64,
        samples: u32,
    ) -> Result<Color, String> {
        if samples == 1 {
            let ray = self.camera.get_ray(u + width / 2.0, v + width / 2.0)?;
            return self.ray_color(&ray);
        }
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();
        for _sample in 0..samples {
            let rng_u: f64 = rng.gen();
            let rng_v: f64 = rng.gen();
            let sample_du = width * rng_u;
            let sample_dv = height * rng_v;
            let (sample_u, sample_v) = (u + sample_du, v + sample_dv);
            let ray = self.camera.get_ray(sample_u, sample_v)?;
            let ray_color = self.ray_color(&ray)?;
            pixel_color += ray_color;
        }
        pixel_color /= samples as f64;
        Ok(pixel_color)
    }

    pub fn uv_color(&self, u: f64, v: f64) -> Result<Color, String> {
        let ray = self.camera.get_ray(u, v)?;
        self.ray_color(&ray)
    }

    pub fn ray_color(&self, ray: &Ray) -> Result<Color, String> {
        let hit = self.world.hit(&ray, 0.0, INFINITY);
        if hit.is_some() {
            let color = (hit.unwrap().normal + ONE) / 2.0;
            return Ok(color);
        }
        let t = 0.5 * (ray.unit_dir().y + 1.0);
        Ok((1.0 - t) * ONE + t * Color::new(0.5, 0.7, 1.0))
    }
}
