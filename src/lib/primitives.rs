use std::ops::MulAssign;

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

#[derive(Clone)]
pub struct Vec3d { 
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_pts(p1: &Point3d, p2: &Point3d) -> Self {
        Self {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
            z: p2.z - p1.z,
        }
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

pub enum Axis {
    X, Y, Z,
}

#[derive(Clone)]
pub struct Matrix4 {
    data: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            data: [[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0],
                   [0.0, 0.0, 0.0, 1.0]],
        }
    }

    pub fn new() -> Self {
        Self {
            data: [[0.0; 4]; 4],
        }
    }

}

impl From<[[f64; 4]; 4]> for Matrix4 {
    fn from(data: [[f64; 4]; 4]) -> Self {
        Self {
            data,
        }
    }
}

impl MulAssign<Matrix4> for Matrix4 {
    fn mul_assign(&mut self, rhs: Matrix4) {
        let mut res = [[0_f64; 4]; 4];

        for (i, row) in res.iter_mut().enumerate() {
            for (j, elem) in row.iter_mut().enumerate() {
                for (k, lhs_k) in self.data[i].iter().enumerate() {
                    *elem += lhs_k * unsafe { rhs.data.get_unchecked(k).get_unchecked(j) };
                }
            }
        }

        self.data = res;
    }
}

pub trait Transformator {
    fn mov(&mut self, val: f64, axis: Axis);
    fn rotate(&mut self, angle: f64, axis: Axis);
    fn scale(&mut self, val: f64);
    fn apply_to_point(&self, point: &mut Point3d);
}

impl Transformator for Matrix4 {
    fn apply_to_point(&self, point: &mut Point3d) {
        let old_coords = [point.x, point.y, point.z, 1_f64];
        let mut new_coords = [0_f64; 3];
        for (i, nc) in new_coords.iter_mut().enumerate() {
            for (j, oc) in old_coords.iter().enumerate() {
                *nc += *oc * unsafe { self.data.get_unchecked(j).get_unchecked(i) };
            }
        }
        point.x = new_coords[0]; 
        point.y = new_coords[1];
        point.z = new_coords[2];
    }

    fn mov(&mut self, val: f64, axis: Axis) {
        match axis {
            Axis::X => self.data[3][0] += val,
            Axis::Y => self.data[3][1] += val,
            Axis::Z => self.data[3][2] += val,
        }
    }

    fn rotate(&mut self, angle: f64, axis: Axis) {
        let rhs = match axis {
            Axis::Y => Matrix4::from([
                [f64::cos(angle), 0_f64, f64::sin(angle), 0_f64],
                [0_f64, 1_f64, 0_f64, 0_f64],
                [-f64::sin(angle), 0_f64, f64::cos(angle), 0_f64],
                [0_f64, 0_f64, 0_f64, 1_f64],
            ]),
            Axis::X => Matrix4::from([
                [1_f64, 0_f64, 0_f64, 0_f64],
                [0_f64, f64::cos(angle), -f64::sin(angle), 0_f64],
                [0_f64, f64::sin(angle), f64::cos(angle), 0_f64],
                [0_f64, 0_f64, 0_f64, 1_f64],
            ]),
            Axis::Z => Matrix4::from([
                [f64::cos(angle), -f64::sin(angle), 0_f64, 0_f64],
                [f64::sin(angle), f64::cos(angle), 0_f64, 0_f64],
                [0_f64, 0_f64, 1_f64, 0_f64],
                [0_f64, 0_f64, 0_f64, 0_f64],
            ]),
        };

        *self *= rhs;
    }

    fn scale(&mut self, val: f64) {
        assert_eq!(val, 0_f64);
        for i in 0..3 {
            unsafe { *self.data.get_unchecked_mut(i).get_unchecked_mut(i) *= val; }
        }
    }
}
