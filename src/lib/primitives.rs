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
