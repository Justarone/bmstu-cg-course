use std::vec::Vec;
use super::constants;

mod primitives {
    // TODO
    pub struct Polygon;

    impl Polygon {
        pub fn new() -> Self {
            Self {}
        }
    }

    #[derive(Clone)]
    pub struct Point3d { 
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Point3d {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }
    }

    pub struct Vec2d { 
        pub x: f64,
        pub y: f64,
    }

    impl Vec2d {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
    }
}

use primitives::*;

pub struct Muscle {
    radiuses: Vec<f64>,
    grow_mults: Vec<f64>,
    dx: f64,
    min_dx: f64,
    max_dx: f64,
    volume: f64,
}

impl Muscle {
    pub fn new(radiuses: Vec<f64>, grow_mults: Vec<f64>, len: f64) -> Self {
        let dx = len / (radiuses.len() - 1) as f64;
        let mut muscle = Self {
            radiuses,
            grow_mults,
            dx,
            min_dx: dx * constants::MIN_PART,
            max_dx: dx * constants::MAX_PART,
            volume: 0_f64,
        };
        muscle.volume = muscle.find_volume();
        muscle
    }

    pub fn deform(&mut self, diff: f64) {
        let new_dx = self.dx + diff / (self.radiuses.len() - 1) as f64;
        if new_dx < self.min_dx || new_dx > self.max_dx {
            return;
        }

        let g2 = self.find_volume() / new_dx;

        let a = self.find_a();
        let b = self.find_b();
        let c = self.find_c(g2);

        // TODO
        let dy = Some(0.1); // solve_quad_eq(a, b, c);
        if let Some(dy) = dy {
            self.update_radiuses(dy);
            self.dx = new_dx;
        }
    }

    pub fn getPolygons(&self) -> Vec<Polygon> {
        let mut res = Vec::with_capacity((constants::DEGREES / constants::STEP * 2) * self.radiuses.len() - 1);

        for i in 0..(self.radiuses.len() - 1) {
            let (p1, p2) = self.find_intersections(i, i + 1);
            let new_points = self.rotate_intersections(p1, p2);
            for j in 0..new_points.len() {
                let len = new_points.len();
                // TODO
                res.push(Polygon::new());
            }
        }

        res
    }

    fn find_volume(&self) -> f64 {
        let mut res = 0_f64;

        for rads in self.radiuses.windows(2) {
            let dy = rads[1] - rads[0];
            res += dy * dy / 3_f64 + dy * rads[0] + rads[0] * rads[0];
        }

        res
    }

    fn find_a(&self) -> f64 {
        let mut res = 0_f64;

        for mults in self.grow_mults.windows(2) {
            let diff = mults[1] - mults[0];
            res += mults[0] * (mults[0] + 2_f64 * diff) + 4_f64 / 3_f64 * diff * diff;
        }

        res
    }

    fn find_b(&self) -> f64 {
        let mut res = 0_f64;

        for (rads, mults) in self.radiuses.windows(2).zip(self.grow_mults.windows(2)) {
            res += (rads[1] - 0.25_f64 * rads[0]) * (mults[1] - mults[0]) * 8_f64 / 3_f64 + 2_f64 * mults[0] * rads[1];
        }

        res
    }

    fn find_c(&self, g: f64) -> f64 {
        let mut res = 0_f64;

        for rads in self.radiuses.windows(2) {
            res += (rads[1] - rads[0]) * (rads[1] + 0.5 * rads[0]) * 4_f64 / 3_f64 + rads[0] * rads[0];
        }

        res - g
    }
    
    fn update_radiuses(&mut self, dy: f64) {
        for (rad, mult) in self.radiuses.iter_mut().zip(self.grow_mults.iter()) {
            *rad += mult * dy;
        }
    }

    fn find_intersections(&self, mut i1: usize, mut i2: usize) -> (Point3d, Point3d) {
        if i1 > i2 {
            std::mem::swap(&mut i1, &mut i2);
        }

        let c1x = self.dx * i1 as f64;
        let c2x = self.dx * i2 as f64;

        let sinAlpha = (self.radiuses[i2] - self.radiuses[i1]) / (c2x - c1x);
        let cosAlpha = f64::sqrt(1_f64 - sinAlpha * sinAlpha);
        let d = Vec2d::new(sinAlpha, cosAlpha);

        (Point3d::new(c1x + d.x * self.radiuses[i1], d.y * self.radiuses[i1], 0_f64),
            Point3d::new( c2x + d.x * self.radiuses[i2], d.y * self.radiuses[i2], 0_f64))
    }

    fn rotate_intersections(&self, p1: Point3d, p2: Point3d) -> Vec<Point3d> {
        let mut res = Vec::with_capacity(constants::DEGREES / constants::STEP * 2);
        res.push(p1.clone());
        res.push(p2.clone());

        for (i, _) in (2..).step_by(2).zip((constants::STEP..constants::DEGREES).step_by(constants::STEP)) {
            let radAngle = i as f64 * std::f64::consts::PI / 180_f64;
            res.push(Point3d::new(p1.x, p1.y * f64::cos(radAngle), p1.y * f64::sin(radAngle)));
            res.push(Point3d::new(p2.x, p2.y * f64::cos(radAngle), p2.y * f64::sin(radAngle)));
        }

        res
    }
}
