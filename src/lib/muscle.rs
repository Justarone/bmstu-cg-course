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

        let dy = solve_quad_eq(a, b, c);
        if let Some(dy) = dy.1 {
            self.update_radiuses(dy);
            self.dx = new_dx;
        }
    }

    fn fill_pn_connectors(&self, points: &mut Vec<Vec<Point3d>>, normals: &mut Vec<Vec<Vec3d>>) {
        for i in 0..(self.radiuses.len() - 1) {
            let (p1, p2) = self.find_intersections(i, i + 1);

            let (mut new_points, mut new_normals) = self.rotate_intersections(p1, p2,
                Point3d::new(self.dx * i as f64, 0_f64, 0_f64), // center of i-th sphere
                Point3d::new(self.dx * (i + 1) as f64, 0_f64, 0_f64)); // center of (i + 1)-th sphere

            for j in 0..2 {
                new_points.push(new_points[j].clone());
                new_normals.push(new_normals[j].clone());
            }

            points.push(new_points);
            normals.push(new_normals);
        }
    }

    fn fill_pn_spheres(&self, points: &mut Vec<Vec<Point3d>>, normals: &mut Vec<Vec<Vec3d>>) {
        for (center, rad) in (0..self.radiuses.len()).map(|val| self.dx * val as f64).zip(self.radiuses.iter()) {
            let (from, step) = (center - rad, 2_f64 * rad / (constants::SPHERE_PARTS - 1) as f64);
            let mut solutions = Vec::with_capacity(constants::SPHERE_PARTS);

            for x in (0..constants::SPHERE_PARTS).map(|i| from + step * i as f64) {
                solutions.push(Point3d::new(x, f64::sqrt(rad - f64::powi(x - center, 2)), 0_f64));
            }

            for (i, pts) in solutions.windows(2).enumerate() {
                let (mut new_points, mut new_normals) = self.rotate_intersections(pts[0].clone(), pts[1].clone(),
                    Point3d::new(self.dx * i as f64, 0_f64, 0_f64), // center of i-th sphere
                    Point3d::new(self.dx * (i + 1) as f64, 0_f64, 0_f64)); // center of (i + 1)-th sphere

                for j in 0..2 {
                    new_points.push(new_points[j].clone());
                    new_normals.push(new_normals[j].clone());
                }

                points.push(new_points);
                normals.push(new_normals);
            }
        }
    }

    fn get_points_and_normals(&self) -> (Vec<Vec<Point3d>>, Vec<Vec<Vec3d>>) {
        let mut points =  Vec::with_capacity(self.radiuses.len() * (constants::SPHERE_PARTS) - 1);
        let mut normals = Vec::with_capacity(self.radiuses.len() * (constants::SPHERE_PARTS) - 1);
        self.fill_pn_connectors(&mut points, &mut normals);
        self.fill_pn_spheres(&mut points, &mut normals);

        (points, normals)
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
            res += (rads[1] - 0.25_f64 * rads[0]) * (mults[1] - mults[0]) * 8_f64 / 3_f64 +
                2_f64 * mults[0] * rads[1];
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

        let sin_alpha = (self.radiuses[i2] - self.radiuses[i1]) / (c2x - c1x);
        let cos_alpha = f64::sqrt(1_f64 - sin_alpha * sin_alpha);
        let d = Vec2d::new(sin_alpha, cos_alpha);

        (Point3d::new(c1x + d.x * self.radiuses[i1], d.y * self.radiuses[i1], 0_f64),
            Point3d::new( c2x + d.x * self.radiuses[i2], d.y * self.radiuses[i2], 0_f64))
    }

    fn rotate_intersections(&self, p1: Point3d, p2: Point3d, c1: Point3d,
        c2: Point3d) -> (Vec<Point3d>, Vec<Vec3d>) {
        let mut points = Vec::with_capacity(constants::DEGREES / constants::STEP * 2);
        let mut normals = Vec::with_capacity(constants::DEGREES / constants::STEP * 2);

        points.push(p1.clone());
        points.push(p2.clone());
        normals.push(Vec3d::from_pts(&c1, &p1));
        normals.push(Vec3d::from_pts(&c2, &p2));

        for angle in (constants::STEP..constants::DEGREES).step_by(constants::STEP) {
            let angle = angle as f64 * std::f64::consts::PI / 180_f64; // convert to radians
            let t1 = Point3d::new(p1.x, p1.y * f64::cos(angle), p1.y * f64::sin(angle));
            let t2 = Point3d::new(p2.x, p2.y * f64::cos(angle), p2.y * f64::sin(angle));

            normals.push(Vec3d::from_pts(&c1, &t1));
            normals.push(Vec3d::from_pts(&c2, &t2));
            points.push(t1);
            points.push(t2);
        }

        (points, normals)
    }
}
