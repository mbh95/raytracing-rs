use crate::vec3::Vec3;
use std::cmp::{max, min};
use std::io::Write;

type Color = Vec3;

fn unit_to_max(x: f64, max_value: u32) -> u32 {
    let i = (x * max_value as f64) as i32;
    min(max(i, 0) as u32, max_value)
}

pub fn write_color(o: &mut dyn Write, c: Color) -> Result<(), String> {
    let normalized = c.norm()?;
    writeln!(
        o,
        "{} {} {}",
        unit_to_max(normalized.x, 255),
        unit_to_max(normalized.y, 255),
        unit_to_max(normalized.z, 255)
    )
    .map_err(|e| e.to_string())
}
