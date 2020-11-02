#[allow(dead_code)]
pub fn dy_stub(grow: bool) -> (Option<f64>, Option<f64>) {
    if grow {
        (Some(0.1), Some(0.1))
    } else {
        (Some(-0.1), Some(-0.1))
    }
}
