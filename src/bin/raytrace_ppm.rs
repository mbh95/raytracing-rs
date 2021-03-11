use rayon::prelude::*;
use raytrace::camera::Camera;
use raytrace::color::{write_color, Color};
use raytrace::hittable_list::HittableList;
use raytrace::math::get_uv;
use raytrace::raytracer::Raytracer;
use raytrace::sphere::Sphere;
use raytrace::vec3::Vec3;
use std::time::SystemTime;

fn main() -> Result<(), String> {
    let st = SystemTime::now();

    // Image
    let image_width = 1280;
    let image_height = 720;
    let samples_per_pixel = 10;

    // let mut buffer = BufferedImage::new(image_width, image_height);

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

    let raytracer: Raytracer = Raytracer::new(&camera, &world);

    let mut buffer = vec![Color::new(0.0, 0.0, 0.0); image_width * image_height];

    // Render
    buffer
        .par_chunks_mut(image_width)
        .enumerate()
        .for_each(|(y, color)| {
            for x in 0..image_width {
                color[x] = [0..samples_per_pixel]
                    .iter()
                    .map(|_sample_num| {
                        let (u, v) =
                            get_uv(x as f64, y as f64, image_width as u32, image_height as u32);
                        raytracer.sample_region(u, v, pixel_width, pixel_height)
                    })
                    .sum::<Color>()
                    / samples_per_pixel as f64;
            }
        });

    eprintln!(
        "Finished rendering in {}ms.",
        st.elapsed().map_err(|_e| "Some error")?.as_millis()
    );

    eprintln!("Writing output.");

    // Output
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
            write_color(&mut std::io::stdout(), buffer[y * image_width + x])?;
        }
    }
    eprintln!("\rScanlines remaining: 0 ({:.1}%)\nDone!", 100.0);
    Ok(())
}
