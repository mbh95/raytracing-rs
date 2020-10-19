use raytrace::color::{write_color, Color};
use regex::{CaptureMatches, Regex};
use std::{env, fs};

fn next(captures: &mut CaptureMatches) -> String {
    String::from(captures.next().unwrap().get(1).unwrap().as_str())
}

fn next_u32(captures: &mut CaptureMatches) -> u32 {
    next(captures).parse::<u32>().unwrap()
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let filename1 = &args[1];
    let filename2 = &args[2];

    let ppm1 = fs::read_to_string(filename1)
        .expect(format!("Something went wrong reading file '{}'", filename1).as_str());
    let body1 = &ppm1[2..];

    let ppm2 = fs::read_to_string(filename2)
        .expect(format!("Something went wrong reading file '{}'", filename1).as_str());
    let body2 = &ppm2[2..];

    let uint_matcher = Regex::new(r"(\d+)").unwrap();
    let mut tokens1 = uint_matcher.captures_iter(body1);
    let mut tokens2 = uint_matcher.captures_iter(body2);

    let width1 = next_u32(&mut tokens1);
    let height1 = next_u32(&mut tokens1);
    let base1 = next_u32(&mut tokens1);

    let width2 = next_u32(&mut tokens2);
    let height2 = next_u32(&mut tokens2);
    let base2 = next_u32(&mut tokens2);

    assert_eq!(width1, width2);
    assert_eq!(height1, height2);

    let w = width1;
    let h = height1;

    // Render delta
    println!("P3");
    println!("{} {}", w, h);
    println!("255");
    for y in (0..h).rev() {
        let remaining = y + 1;
        let percent_done = 100.0 * (h - remaining) as f64 / (h) as f64;
        eprint!(
            "\rScanlines remaining: {} ({:.1}%)",
            remaining, percent_done
        );
        for _x in 0..w {
            let r1 = next_u32(&mut tokens1) as f64 / base1 as f64;
            let g1 = next_u32(&mut tokens1) as f64 / base1 as f64;
            let b1 = next_u32(&mut tokens1) as f64 / base1 as f64;

            let r2 = next_u32(&mut tokens2) as f64 / base2 as f64;
            let g2 = next_u32(&mut tokens2) as f64 / base2 as f64;
            let b2 = next_u32(&mut tokens2) as f64 / base2 as f64;

            let dr = r1 - r2;
            let dg = g1 - g2;
            let db = b1 - b2;

            let r = dr.abs();
            let g = dg.abs();
            let b = db.abs();

            let color = Color::new(r, g, b);
            write_color(&mut std::io::stdout(), color)?;
        }
    }
    eprintln!("\rScanlines remaining: 0 ({:.1}%)\nDone!", 100.0);
    Ok(())
}
