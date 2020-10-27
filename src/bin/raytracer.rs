use rand::Rng;
use raytrace::camera::Camera;
use raytrace::color::{write_color, Color};
use raytrace::hittable::Hittable;
use raytrace::hittable_list::HittableList;
use raytrace::ray::Ray;
use raytrace::sphere::Sphere;
use raytrace::vec3::{Vec3, ONE};
use std::f64::INFINITY;

fn map_to_range(val: f64, src_min: f64, src_max: f64, dst_min: f64, dst_max: f64) -> f64 {
    if src_min > src_max || dst_min > dst_max {
        panic!(
            "Invalid range specified in map to range: ({}, {}), ({}, {})",
            src_min, src_max, dst_min, dst_max
        );
    }
    ((val - src_min) / (src_max - src_min)) * (dst_max - dst_min) + dst_min
}
fn get_uv(pixel_x: f64, pixel_y: f64, image_width: u32, image_height: u32) -> (f64, f64) {
    (
        map_to_range(pixel_x, 0.0, (image_width - 1) as f64, -1.0, 1.0),
        map_to_range(pixel_y, 0.0, (image_height - 1) as f64, -1.0, 1.0),
    )
}

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
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_height = 400u32;
    let image_width = (aspect_ratio * image_height as f64) as u32;
    let samples_per_pixel = 100;

    // Camera
    let camera = Camera::new();

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
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for sample in 0..samples_per_pixel {
                let sample_dx: f64 = rng.gen();
                let sample_dy: f64 = rng.gen();
                let (u, v) = get_uv(
                    (x as f64) + sample_dx,
                    (y as f64) + sample_dy,
                    image_width,
                    image_height,
                );

                let ray = camera.get_ray(u, v)?;

                let ray_color = ray_color(&ray, &world)?;
                pixel_color += ray_color;
            }
            pixel_color /= samples_per_pixel as f64;
            write_color(&mut std::io::stdout(), pixel_color)?;
        }
    }
    eprintln!("\rScanlines remaining: 0 ({:.1}%)\nDone!", 100.0);
    Ok(())
}
