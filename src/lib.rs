#[allow(dead_code)]
pub mod nelder_mead;

type Parameters = Vec<f64>;

pub fn distance(x: &Parameters, y: &Parameters) -> f64 {
    x.iter().zip(y.iter()).map(|(xx, yy)| (xx - yy) * (xx - yy)).sum::<f64>().sqrt()
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
