use raytrace::color::{write_color, Color};
use raytrace::ray::Ray;
use raytrace::vec3::{Vec3, ONE, ZERO};

fn hit_sphere(sphere_center: &Vec3, sphere_radius: f64, ray: &Ray) -> f64 {
    // Solving ||t*ray.direction - (sphere_center - ray.origin)|| = sphere_radius
    let d = ray.unit_dir();
    let s = *sphere_center - ray.origin;
    // ||t*d - S||^2 = R^2 = (t*d - S) dot (t*d - s) = t^2*(d dot d) + t*2(d dot -S) + (S dot S)
    // Use quadratic equation:
    // let a = d.dot(d);
    // let b = 2.0 * d.dot(&-s);
    // let c = s.dot(&s) - (sphere_radius * sphere_radius);
    // let determinant = b * b - 4.0 * a * c;
    // Notice a = 1 and b is a multiple of 2:
    let half_b = d.dot(&-s);
    let c = s.len_sq() - (sphere_radius * sphere_radius);
    let determinant = half_b * half_b - c;
    if determinant < 0.0 {
        return -1.0;
    }
    -half_b - determinant.sqrt()
}

fn ray_color(ray: &Ray) -> Result<Color, String> {
    let sphere_radius = 0.5;
    let sphere_center = Vec3::new(0.0, 0.0, -1.0);
    let hit_t = hit_sphere(&sphere_center, sphere_radius, ray);
    if hit_t >= 0.0 {
        let hit_point = ray.at(hit_t);
        let normal = (hit_point - sphere_center).norm()?;
        let color = (normal + ONE) / 2.0;
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

            let color = ray_color(&ray)?;
            write_color(&mut std::io::stdout(), color)?;
        }
    }
    eprintln!("\rScanlines remaining: 0 ({:.1}%)\nDone!", 100.0);
    Ok(())
}
