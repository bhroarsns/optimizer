use super::Parameters;
use super::is_indistinguishable;
use std::io::{BufWriter, Write};

pub fn shift(target: &Parameters, center: &Parameters, coef: f64) -> Parameters {
    target.iter().zip(center.iter()).map(|(t, c)| {
        c + coef * (t - c)
    }).collect()
}

pub fn optimize<T: Fn(&Parameters) -> f64, W: Write>(init: &Parameters, cost_function: T, epsilon: f64, lambda: Vec<f64>, log: Option<BufWriter<W>>) -> Parameters {
    let dimension = init.len();

    let mut simplex: Vec<Parameters> = (0..=dimension).map(|i| {
        if i == 0 {
            init.iter().map(|x| *x).collect()
        } else {
            init.iter().enumerate().map(|(j, x)| {
                if j == (i - 1) {
                    x + lambda[j]
                } else {
                    *x
                }
            }).collect()
        }
    }).collect();

    match log {
        Some(mut out) => {
            loop {
                simplex.sort_unstable_by(|a, b| cost_function(&a).partial_cmp(&cost_function(&b)).unwrap());

                let f_x_0 = cost_function(&simplex[0]);
                let f_x_n = cost_function(&simplex[dimension]);

                write!(out, "{} {}", f_x_0, f_x_n).expect("");

                if (f_x_n - f_x_0) / f_x_0 < epsilon {
                    writeln!(out, " iteration stopped because the percision rate({}) reached the required value.", (f_x_n - f_x_0) / f_x_0).expect("");
                    break;
                }
                
                if is_indistinguishable(&simplex[0], &simplex[dimension], 0.5) {
                    writeln!(out, " iteration stopped because two vertices of the simplex got indistinguishable.").expect("");
                    break;
                }

                let x_o = simplex.iter().take(dimension).fold(
                    (0..dimension).map(|_| 0.0).collect(),
                    |tmp_sum: Parameters, vertex| {
                        tmp_sum.iter().zip(vertex.iter()).map(|(a, b)| a + b).collect()
                    }
                ).into_iter().map(|sum| sum / dimension as f64).collect();

                let x_r = shift(&simplex[dimension], &x_o, -1.0);
                let f_x_r = cost_function(&x_r);

                if f_x_r < f_x_0 {
                    let x_e = shift(&x_r, &x_o, 2.0);
                    if cost_function(&x_e) < f_x_r {
                        simplex.pop();
                        simplex.push(x_e);
                        writeln!(out, "-> expand").expect("");
                    } else {
                        simplex.pop();
                        simplex.push(x_r);
                        writeln!(out, "-> reflect1").expect("");
                    }
                } else {
                    if f_x_r < cost_function(&simplex[dimension - 1]) {
                        simplex.pop();
                        simplex.push(x_r);
                        writeln!(out, "-> reflect2").expect("");
                    } else {
                        let x_c = shift(&simplex[dimension], &x_o, 0.5);
                        if cost_function(&x_c) < f_x_n {
                            simplex.pop();
                            simplex.push(x_c);
                            writeln!(out, "-> contract1").expect("");
                        } else {
                            for i in 1..=dimension {
                                simplex[i] = shift(&simplex[i], &simplex[0], 0.5);
                            }
                            writeln!(out, "-> contract2").expect("");
                        }
                    }
                }
            }
        },
        None => {
            loop {
                simplex.sort_unstable_by(|a, b| cost_function(&a).partial_cmp(&cost_function(&b)).unwrap());

                let f_x_0 = cost_function(&simplex[0]);
                let f_x_n = cost_function(&simplex[dimension]);

                if (f_x_n - f_x_0) / f_x_0 < epsilon {
                    break;
                }
                
                if is_indistinguishable(&simplex[0], &simplex[dimension], 0.5) {
                    break;
                }

                let x_o = simplex.iter().take(dimension).fold(
                    (0..dimension).map(|_| 0.0).collect(),
                    |tmp_sum: Parameters, vertex| {
                        tmp_sum.iter().zip(vertex.iter()).map(|(a, b)| a + b).collect()
                    }
                ).into_iter().map(|sum| sum / dimension as f64).collect();

                let x_r = shift(&simplex[dimension], &x_o, -1.0);
                let f_x_r = cost_function(&x_r);

                if f_x_r < f_x_0 {
                    let x_e = shift(&x_r, &x_o, 2.0);
                    if cost_function(&x_e) < f_x_r {
                        simplex.pop();
                        simplex.push(x_e);
                    } else {
                        simplex.pop();
                        simplex.push(x_r);
                    }
                } else {
                    if f_x_r < cost_function(&simplex[dimension - 1]) {
                        simplex.pop();
                        simplex.push(x_r);
                    } else {
                        let x_c = shift(&simplex[dimension], &x_o, 0.5);
                        if cost_function(&x_c) < f_x_n {
                            simplex.pop();
                            simplex.push(x_c);
                        } else {
                            for i in 1..=dimension {
                                simplex[i] = shift(&simplex[i], &simplex[0], 0.5);
                            }
                        }
                    }
                }
            }
        }
    }

   simplex[0].clone()
}