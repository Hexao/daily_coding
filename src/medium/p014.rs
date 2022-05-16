/// The area of a circle is defined as `πr^2`. Estimate `π` to `3`
/// decimal places using a Monte Carlo method.
///
/// Hint: The basic equation of a circle is x2 + y2 = r2.
pub fn compute_pi() -> f64 {
    const STEP: u64 = 999_999;
    let mut inside: u64 = 0;
    let mut out: u64 = 0;

    for py in (0..=STEP).rev() {
        let y = (py as f64 / STEP as f64).powi(2);
        inside += out;

        for px in out..=STEP {
            let x = (px as f64 / STEP as f64).powi(2);
            if x + y < 1.0 {
                inside += 1;
            } else {
                out = px;
                break;
            }
        }
    }

    (inside as f64 / STEP.pow(2) as f64) * 4.0
}
