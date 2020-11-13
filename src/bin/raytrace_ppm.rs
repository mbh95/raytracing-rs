use raytrace::camera::Camera;
use raytrace::color::write_color;
use raytrace::hittable_list::HittableList;
use raytrace::math::get_uv;
use raytrace::raytracer::Raytracer;
use raytrace::sphere::Sphere;
use raytrace::vec3::Vec3;

fn main() -> Result<(), String> {
    let mut rng = rand::thread_rng();

    // Image
    let image_width = 640u32;
    let image_height = 480u32;
    let samples_per_pixel = 10;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let (pixel_width, pixel_height) = (2.0 / image_width as f64, 2.0 / image_height as f64);
    // Camera`
    let camera = Camera::new(aspect_ratio);

    // World
    let mut world = HittableList::new();
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    world.add(&sphere1);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&sphere2);

    let mut raytracer: Raytracer = Raytracer::new(&camera, &world, &mut rng);

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
            let (u, v) = get_uv(x as f64, y as f64, image_width, image_height);
            let pixel_color =
                raytracer.region_color(u, v, pixel_width, pixel_height, samples_per_pixel)?;
            write_color(&mut std::io::stdout(), pixel_color)?;
        }
    }
    eprintln!("\rScanlines remaining: 0 ({:.1}%)\nDone!", 100.0);
    Ok(())
}
