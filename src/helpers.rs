pub fn f64_equal(a: f64, b: f64) -> bool {
    let margin = f64::EPSILON;
    (a - b).abs() < margin
}

pub fn f32_equal(a: f32, b: f32) -> bool {
    let margin = f32::EPSILON;
    (a - b).abs() < margin
}
