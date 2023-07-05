pub fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> f32 {
    let x = x1.abs_diff(x2) as f32;
    let y = y1.abs_diff(y2) as f32;

    (x.powi(2) + y.powi(2)).sqrt()
}
