use std::io::BufReader;
use std::fs::File;
use std::env;
use serde::{ Serialize, Deserialize };
use std::f64;

use super::prelude::*;

pub fn solve_quad_eq(a: f64, b: f64, c: f64) -> (Option<f64>, Option<f64>) {
    if relative_eq!(a, 0_f64) {
        if relative_eq!(b, 0_f64) {
            (None, None)
        } else {
            (Some(c / b), None)
        }
    } else {
        let det = b * b - 4_f64 * a * c;
        if relative_eq!(det, 0_f64) {
            (Some(-b / 2_f64 * a), None) 
        } else if det < 0_f64 { 
            (None, None)
        } else {
            let dsqrt = f64::sqrt(det);
            (Some((-b - dsqrt) / 2_f64 / a), Some((-b + dsqrt) / 2_f64 / a))
        }
    }
}

pub fn cycle_extend<T: Clone>(arr: &mut Vec<T>, n: usize) {
    for i in 0..n {
        arr.push(arr[i].clone());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub radiuses: Vec<f64>,
    pub grow_mults: Vec<f64>,
    pub len: f64,
}

pub fn read_from_config() -> Config {
    let mut config_path = env::current_dir().expect("Current directory");
    for elem in constants::RELATIVE_CONF_PATH.iter() {
        config_path.push(elem);
    }
    let reader = File::open(config_path.to_str().expect("File to string")).expect("Open config file");
    let reader = BufReader::new(reader);
    serde_yaml::from_reader(reader).expect("Data from config")
}

#[cfg(test)]
mod tests {
    use super::solve_quad_eq;
    
    #[test]
    fn a0_no_sols() {
        assert_eq!((None, None), solve_quad_eq(0_f64, 0_f64, 1_f64));
    }

    #[test]
    fn a0_inf_sols() {
        assert_eq!((None, None), solve_quad_eq(0_f64, 0_f64, 0_f64));
    }

    #[test]
    fn a0_one_sol() {
        assert_eq!((Some(0.125_f64), None), solve_quad_eq(0_f64, 16_f64, 2_f64));
    }

    #[test]
    fn bad_det() {
        assert_eq!((None, None), solve_quad_eq(1_f64, 1_f64, 1_f64));
    }

    #[test]
    fn full() {
        assert_eq!((Some(-2_f64), None), solve_quad_eq(1_f64, 4_f64, 4_f64));
    }
    
    #[test]
    fn usual() {
        assert_eq!((Some(3_f64), Some(6_f64)), solve_quad_eq(1_f64, -9_f64, 18_f64));
    }
}
