use super::prelude::*;
use std::vec::Vec;

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
            volume: f64::default(),
        };
        muscle.volume = muscle.find_volume();
        muscle
    }

    #[allow(dead_code)]
    pub fn len(&self) -> f64 {
        self.dx * (self.radiuses.len() - 1) as f64
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

        //let dy = dy_stub(self.dx > new_dx);
        let dy = solve_quad_eq(a, b, c);
        if let Some(dy) = dy.1 {
            self.update_radiuses(dy);
            self.dx = new_dx;
        }
    }

    fn fill_pn_connectors(
        &self,
        points: &mut Vec<Vec<Point3d>>,
        normal2points: &mut Vec<Vec<Point3d>>,
    ) {
        for i in 0..(self.radiuses.len() - 1) {
            let (p1, p2) = self.find_intersections(i, i + 1);

            let (mut new_points, mut new_norm2points) = rotate_intersections(
                &[p1, p2],
                &[
                Point3d::new(self.dx * i as f64, 0_f64, 0_f64), // center of i-th sphere
                Point3d::new(self.dx * (i + 1) as f64, 0_f64, 0_f64),
                ], // center of (i + 1)-th sphere
                constants::MUSCLE_STEP,
            );
            cycle_extend(&mut new_points, 2);
            cycle_extend(&mut new_norm2points, 2);

            points.push(new_points);
            normal2points.push(new_norm2points);
        }
    }

    fn fill_pn_spheres(
        &self,
        points: &mut Vec<Vec<Point3d>>,
        normal2points: &mut Vec<Vec<Point3d>>,
    ) {
        let index_arr = [0, self.radiuses.len() - 1];
        for (center, rad) in index_arr
            .iter()
                .map(|&index| (self.dx * index as f64, self.radiuses[index]))
                {
                    add_uv_sphere(points, normal2points, center, rad);
                }
    }

    pub fn get_points_and_normals(&self) -> (Vec<Vec<Point3d>>, Vec<Vec<Point3d>>) {
        let mut points = Vec::with_capacity(self.radiuses.len() * (constants::SPHERE_PARTS) - 1);
        let mut normal2points =
            Vec::with_capacity(self.radiuses.len() * (constants::SPHERE_PARTS) - 1);
        self.fill_pn_connectors(&mut points, &mut normal2points);
        self.fill_pn_spheres(&mut points, &mut normal2points);

        (points, normal2points)
    }

    // volume divided by pi
    fn find_volume(&self) -> f64 {
        let mut res = 0_f64;

        for rads in self.radiuses.windows(2) {
            let dy = rads[1] - rads[0];
            res += dy * dy / 3_f64 + dy * rads[0] + rads[0] * rads[0];
        }

        res * self.dx
    }

    fn find_a(&self) -> f64 {
        let mut res = 0_f64;

        for mults in self.grow_mults.windows(2) {
            res += mults[1] * mults[1] - 5_f64 * mults[0] * mults[1] + 7_f64 * mults[0] * mults[0];
        }

        res / 3_f64
    }

    fn find_b(&self) -> f64 {
        let mut res = 0_f64;

        for (rads, mults) in self.radiuses.windows(2).zip(self.grow_mults.windows(2)) {
            res += mults[1] * rads[0] + mults[0] * rads[1];
        }

        res
    }

    fn find_c(&self, g: f64) -> f64 {
        let mut res = 0_f64;

        for rads in self.radiuses.windows(2) {
            res += 1_f64 / 3_f64 * f64::powi(rads[1] - rads[0], 2) + rads[0] * rads[1];
        }

        res - g
    }

    fn update_radiuses(&mut self, dy: f64) {
        for (rad, mult) in self.radiuses.iter_mut().zip(self.grow_mults.iter()) {
            *rad += mult * dy;
        }
    }

    #[allow(dead_code)]
    fn find_rad_intersections(&self, mut i1: usize, mut i2: usize) -> (Point3d, Point3d) {
        if i1 > i2 {
            std::mem::swap(&mut i1, &mut i2);
        }

        let c1x = self.dx * i1 as f64;
        let c2x = self.dx * i2 as f64;

        let sin_alpha = (self.radiuses[i2] - self.radiuses[i1]) / (c2x - c1x);
        let cos_alpha = f64::sqrt(1_f64 - sin_alpha * sin_alpha);
        let d = Vec2d::new(sin_alpha, cos_alpha);

        (
            Point3d::new(
                c1x + d.x * self.radiuses[i1],
                d.y * self.radiuses[i1],
                0_f64,
            ),
            Point3d::new(
                c2x + d.x * self.radiuses[i2],
                d.y * self.radiuses[i2],
                0_f64,
            ),
        )
    }

    fn find_intersections(&self, mut i1: usize, mut i2: usize) -> (Point3d, Point3d) {
        if i1 > i2 {
            std::mem::swap(&mut i1, &mut i2);
        }

        (
            Point3d::new(self.dx * i1 as f64, self.radiuses[i1], 0_f64),
            Point3d::new(self.dx * i2 as f64, self.radiuses[i2], 0_f64),
        )
    }
}
