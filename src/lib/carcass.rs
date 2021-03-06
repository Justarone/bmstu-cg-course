use super::prelude::*;

pub struct Carcass {
    data: [[f64; 2]; 2],
    thickness: f64,
    cur_len: f64,
}

impl Carcass {
    pub fn new(data: [[f64; 2]; 2], thickness: f64, cur_len: f64) -> Self {
        Self {
            data,
            thickness,
            cur_len,
        }
    }

    pub fn data(&self) -> [[f64; 2]; 2] {
        self.data
    }

    pub fn check_diff(&self, diff: f64) -> bool {
        self.cur_len + diff < self.data[0][1] + self.data[1][0] // max
            && self.cur_len + diff >
            f64::sqrt(f64::abs(f64::powi(self.data[0][1], 2) - f64::powi(self.data[1][0], 2)))
            // min
    }

    #[allow(dead_code)]
    pub fn bounder(&self) -> Box<dyn Fn(f64) -> f64> {
        let a1 = -angle_from_triangle(self.data[1][0], self.data[0][1], self.cur_len);
        let median = self.data[0][1] * f64::cos(a1);
        let a2 = angle_from_triangle(self.data[0][1], self.data[1][0], self.cur_len);
        let b = -self.cur_len * a2;
        let thickness = self.thickness;
        Box::new(move |x| {
            if x <= median {
                -(a1 * x - thickness / f64::cos(std::f64::consts::PI + f64::atan(a1)))
            } else {
                -(a2 * x + b - thickness / f64::cos(std::f64::consts::PI - f64::atan(a2)))
            }
        })
    }

    pub fn deform(&mut self, diff: f64) {
        self.cur_len += diff;
    }

    pub fn get_points_and_normals(&self) -> (Vec<Vec<Point3d>>, Vec<Vec<Point3d>>) {
        let (mut points, mut normals) = self.process_part1();
        let (mut new_points, mut new_normals) = self.process_part2();
        points.append(&mut new_points);
        normals.append(&mut new_normals);
        (points, normals)
    }

    fn process_part1(&self) -> (Vec<Vec<Point3d>>, Vec<Vec<Point3d>>) {
        let (mut points, mut normals) = (Vec::new(), Vec::new());
        let len = self.data[0].iter().fold(0_f64, |val, elem| val + elem);
        self.create_tube(&mut points, &mut normals, len);
        add_uv_sphere(&mut points, &mut normals, 0_f64, self.thickness);
        add_uv_sphere(&mut points, &mut normals, len, self.thickness);
        let angle = -angle_from_triangle(self.data[1][0], self.data[0][1], self.cur_len);

        let mut matrix = Matrix4::identity();
        matrix.mov(-self.data[0][0], Axis::X);
        matrix.rotate(angle, Axis::Z);

        for (p_groups, n_groups) in points.iter_mut().zip(normals.iter_mut()) {
            for (p, n) in p_groups.iter_mut().zip(n_groups.iter_mut()) {
                matrix.apply_to_point(p);
                matrix.apply_to_point(n);
            }
        }

        (points, normals)
    }

    fn process_part2(&self) -> (Vec<Vec<Point3d>>, Vec<Vec<Point3d>>) {
        let (mut points, mut normals) = (Vec::new(), Vec::new());
        let len = self.data[1].iter().fold(0_f64, |val, elem| val + elem);
        self.create_tube(&mut points, &mut normals, len);
        add_uv_sphere(&mut points, &mut normals, len, self.thickness);
        let angle = angle_from_triangle(self.data[0][1], self.data[1][0], self.cur_len);

        let mut matrix = Matrix4::identity();
        matrix.mov(-self.data[1][0], Axis::X);
        matrix.rotate(angle, Axis::Z);
        matrix.mov(self.cur_len, Axis::X);

        for (p_groups, n_groups) in points.iter_mut().zip(normals.iter_mut()) {
            for (p, n) in p_groups.iter_mut().zip(n_groups.iter_mut()) {
                matrix.apply_to_point(p);
                matrix.apply_to_point(n);
            }
        }

        (points, normals)
    }

    fn create_tube(
        &self,
        points: &mut Vec<Vec<Point3d>>,
        normals: &mut Vec<Vec<Point3d>>,
        len: f64,
    ) {
        let (mut tube_points, mut tube_normals) = rotate_intersections(
            &[
            Point3d::new(0_f64, self.thickness, 0_f64),
            Point3d::new(len, self.thickness, 0_f64),
            ],
            &[
            Point3d::new(0_f64, 2_f64 * self.thickness, 0_f64),
            Point3d::new(len, 2_f64 * self.thickness, 0_f64),
            ],
            constants::CARCASS_STEP,
        );
        cycle_extend(&mut tube_points, 2);
        cycle_extend(&mut tube_normals, 2);
        points.push(tube_points);
        normals.push(tube_normals);
    }
}
