pub fn f64_equal(a: f64, b: f64) -> bool {
    let margin = f64::EPSILON;
    (a - b).abs() < margin
}
