use std::cmp::{max, min};


fn unit_to_max(x: f64, max_value: u32) -> u32 {
    let i = (x * max_value as f64) as i32;
    min(max(i, 0) as u32, max_value)
}

fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for y in (0..image_height).rev() {
        let remaining = y + 1;
        let percent_done = 100.0 * (image_height - remaining) as f64 / (image_height) as f64;
        eprint!("\rScanlines remaining: {} ({:.1}%)", remaining, percent_done);
        for x in 0..image_width {
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = unit_to_max(r, 255);
            let ig = unit_to_max(g, 255);
            let ib = unit_to_max(b, 255);

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\rScanlines remaining: 0 ({:.1}%)\nDone!", 100.0);
}
