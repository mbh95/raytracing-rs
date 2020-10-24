use raytrace::color::{write_color, Color};
use raytrace::hittable::Hittable;
use raytrace::hittable_list::HittableList;
use raytrace::ray::Ray;
use raytrace::sphere::Sphere;
use raytrace::vec3::{Vec3, ONE, ZERO};
use std::f64::INFINITY;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Result<Color, String> {
    let hit = world.hit(ray, 0.0, INFINITY);
    if hit.is_some() {
        let color = (hit.unwrap().normal + ONE) / 2.0;
        return Ok(color);
    }
    let t = 0.5 * (ray.unit_dir().y + 1.0);
    Ok((1.0 - t) * ONE + t * Color::new(0.5, 0.7, 1.0))
}

fn main() -> Result<(), String> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_height = 400u32;
    let image_width = (aspect_ratio * image_height as f64) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = ZERO;
    let center_ray_end = Vec3::new(0.0, 0.0, -focal_length);
    let top_right_from_center = Vec3::new(viewport_width / 2.0, viewport_height / 2.0, 0.0);

    // World
    let mut world = HittableList::new();
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    world.add(&sphere1);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&sphere2);

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for y in (0..image_height).rev() {
        let remaining = y + 1;
        let percent_done = 100.0 * (image_height - remaining) as f64 / (image_height) as f64;
        eprint!(
            "\rScanlines remaining: {} ({:.1}%)",
            remaining, percent_done
        );
        for x in 0..image_width {
            let u = (2.0 * x as f64 / (image_width - 1) as f64) - 1.0;
            let v = (2.0 * y as f64 / (image_height - 1) as f64) - 1.0;
            let uv = Vec3::new(u, v, 0.0);
            let ray = Ray::new(origin, center_ray_end + uv * top_right_from_center - origin)?;

            let color = ray_color(&ray, &world)?;
            write_color(&mut std::io::stdout(), color)?;
        }
    }
    eprintln!("\rScanlines remaining: 0 ({:.1}%)\nDone!", 100.0);
    Ok(())
}
