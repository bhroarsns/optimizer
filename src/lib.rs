#[allow(dead_code)]
pub mod nelder_mead;

type Parameters = Vec<f64>;
use std::f64::EPSILON;

pub fn is_indistinguishable(x: &Parameters, y: &Parameters, coef: f64) -> bool {
    x.iter().zip(y.iter()).map(|(xx, yy)| if *xx == 0.0 {(xx - yy) * coef} else {(xx - yy) * coef / xx}).any(|d| d.abs() < coef * EPSILON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift() {
        let target = vec![1.0, 2.0];
        let center = vec![0.0, 0.0];
        let coef = 2.0;
        assert_eq!(vec![2.0, 4.0], nelder_mead::shift(&target, &center, coef));
    }

    #[test]
    fn bug_fix() {
    }
}
