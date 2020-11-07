use serde::{Deserialize, Serialize};
use std::env;
use std::f64;
use std::fs::File;
use std::io::BufReader;

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
            (
                Some((-b - dsqrt) / 2_f64 / a),
                Some((-b + dsqrt) / 2_f64 / a),
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MuscleConfig {
    pub radiuses: Vec<f64>,
    pub grow_mults: Vec<f64>,
    pub len: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarcassConfig {
    pub data: [[f64; 2]; 2],
    pub thickness: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub muscle_config: MuscleConfig,
    pub carcass_config: CarcassConfig,
}

pub fn read_from_config() -> Config {
    let mut config_path = env::current_dir().expect("Current directory");
    for elem in constants::RELATIVE_CONF_PATH.iter() {
        config_path.push(elem);
    }
    let reader =
        File::open(config_path.to_str().expect("File to string")).expect("Open config file");
    let reader = BufReader::new(reader);
    serde_yaml::from_reader(reader).expect("Data from config")
}

pub fn cycle_extend<T: Clone>(arr: &mut Vec<T>, n: usize) {
    for i in 0..n {
        arr.push(arr[i].clone());
    }
}

pub fn add_uv_sphere(
    points: &mut Vec<Vec<Point3d>>,
    normal2points: &mut Vec<Vec<Point3d>>,
    center: f64,
    rad: f64,
) {
    let from = center - rad;
    let step = 2_f64 * rad / (constants::SPHERE_PARTS - 1) as f64;
    let mut solutions = Vec::with_capacity(constants::SPHERE_PARTS);

    for x in (0..constants::SPHERE_PARTS).map(|i| from + step * i as f64) {
        let y = f64::sqrt(rad * rad - f64::powi(x - center, 2));
        solutions.push(Point3d::new(x, y, 0_f64));
    }

    for pts in solutions.windows(2) {
        let cpoint = Point3d::new(center, 0_f64, 0_f64);
        let (mut new_points, mut new_norm2points) =
            rotate_intersections(pts, &[cpoint, cpoint], constants::SPHERE_STEP);
        cycle_extend(&mut new_points, 2);
        cycle_extend(&mut new_norm2points, 2);

        points.push(new_points);
        normal2points.push(new_norm2points);
    }
}

pub fn rotate_intersections(
    pts: &[Point3d],
    centers: &[Point3d],
    step: usize,
) -> (Vec<Point3d>, Vec<Point3d>) {
    let mut points = Vec::with_capacity(constants::DEGREES / step * 2);
    let mut normal2points = Vec::with_capacity(constants::DEGREES / step * 2);

    for angle in (0..=constants::DEGREES)
        .step_by(step)
            .map(|angle| angle as f64 * std::f64::consts::PI / 180_f64)
            {
                for (p, c) in pts.iter().zip(centers.iter()) {
                    let t = Point3d::new(p.x, p.y * f64::cos(angle), p.y * f64::sin(angle));
                    normal2points.push(Point3d::new(
                            2.0 * t.x - c.x,
                            2.0 * t.y - c.y,
                            2.0 * t.z - c.z,
                    ));
                    points.push(t);
                }
            }

    (points, normal2points)
}

pub fn angle_from_triangle(a: f64, b: f64, c: f64) -> f64 {
    let cos = (b * b + c * c - a * a) / (2_f64 * b * c);
    f64::acos(cos)
}
