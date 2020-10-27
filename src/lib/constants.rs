use super::prelude::Vec3d;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const BITS_PER_COLOR: i32 = 8;
pub const HAS_ALPHA: bool = true;

pub const MIN_PART: f64 = 0.4;
pub const MAX_PART: f64 = 2.5;

pub const DEGREES: usize = 360;
pub const STEP: usize = 2;
pub const SPHERE_PARTS: usize = 10;

pub const ROTATE_VAL: f64 = 0.5;
pub const MOVE_VAL: f64 = 5.0;
pub const SCALE_VAL: f64 = 0.8;

pub const ATOM_DIFF: f64 = 1.0;

pub const MIN_Z: f64 = f64::MIN;
pub const DEFAULT_COLOR: u32 = 0x1E1E1EFF;
pub const MUSCLE_COLOR: u32 = 0x800000FF;

// this light vector must be normalized and it must direct to light source
pub const LIGHT_SOURCE_DIRECTION: Vec3d = Vec3d { 
    x: 0.57735,
    y: -0.57735,
    z: -0.57735,
};


pub mod keys {
    // ROTATIONS
    pub const H: u16 = 43; // rotate Y (left)
    pub const L: u16 = 46; // (right)
    pub const J: u16 = 44; // rotate X (down)
    pub const K: u16 = 45; // (up)
    pub const F: u16 = 41; // rotate Z (clockwise)
    pub const T: u16 = 28; // (otherclock-wise)

    // MOVEMENTS
    pub const A: u16 = 38; // move (left)
    pub const S: u16 = 39; // (down)
    pub const D: u16 = 40; // (right)
    pub const W: u16 = 25; // (up)
    pub const Q: u16 = 24; // (top)
    pub const E: u16 = 26; // (bottom)

    // SCALING
    pub const P: u16 = 33; // scale (up)
    pub const M: u16 = 58; // (down)

    // DEFORMATIONS
    pub const X: u16 = 53; // (shorten)
    pub const V: u16 = 55; // (lengthen)
}
