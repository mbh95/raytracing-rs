pub fn get_uv(pixel_x: f64, pixel_y: f64, image_width: u32, image_height: u32) -> (f64, f64) {
    (
        map_to_range(pixel_x, 0.0, (image_width - 1) as f64, -1.0, 1.0),
        map_to_range(pixel_y, 0.0, (image_height - 1) as f64, -1.0, 1.0),
    )
}

pub fn map_to_range(val: f64, src_min: f64, src_max: f64, dst_min: f64, dst_max: f64) -> f64 {
    if src_min > src_max || dst_min > dst_max {
        panic!(
            "Invalid range specified in map to range: ({}, {}), ({}, {})",
            src_min, src_max, dst_min, dst_max
        );
    }
    ((val - src_min) / (src_max - src_min)) * (dst_max - dst_min) + dst_min
}
