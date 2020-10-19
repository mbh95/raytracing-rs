mod color;
mod ray;
mod vec3;

use crate::color::write_color;
use crate::vec3::Vec3;

fn main() -> Result<(), String> {
    let image_width = 256;
    let image_height = 256;
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
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.25;

            write_color(&mut std::io::stdout(), Vec3::new(r, g, b))?;
        }
    }
    eprintln!("\rScanlines remaining: 0 ({:.1}%)\nDone!", 100.0);
    Ok(())
}
