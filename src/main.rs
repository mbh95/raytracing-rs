mod vec3;

use crate::vec3::Vec3;
use std::cmp::{max, min};
use std::io::Write;

type Color = Vec3;

fn unit_to_max(x: f64, max_value: u32) -> u32 {
    let i = (x * max_value as f64) as i32;
    min(max(i, 0) as u32, max_value)
}

fn write_color(o: &mut dyn Write, c: Color) -> Result<(), String> {
    writeln!(
        o,
        "{} {} {}",
        unit_to_max(c.x, 255),
        unit_to_max(c.y, 255),
        unit_to_max(c.z, 255)
    )
    .map_err(|e| e.to_string())
}

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
