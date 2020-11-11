use super::prelude::Vec3d;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const BITS_PER_COLOR: i32 = 8;
pub const HAS_ALPHA: bool = true;

pub const MIN_PART: f64 = 0.4;
pub const MAX_PART: f64 = 2.5;

pub const NEGATIVE_Z_PROJECTION: f64 = -0.2;

pub const DEGREES: usize = 360;

pub const MUSCLE_STEP: usize = 20;
pub const CARCASS_STEP: usize = 45;
pub const SPHERE_STEP: usize = 20;

pub const SPHERE_PARTS: usize = 8;

pub const ROTATE_VAL: f64 = 0.1;
pub const MOVE_VAL: f64 = 5.0;
pub const SCALE_VAL: f64 = 1.25;

pub const ATOM_DIFF: f64 = 2.0;

pub const MIN_Z: f64 = f64::MIN;
pub const DEFAULT_COLOR: u32 = 0x1E1E1EFF;
pub const CARCASS_COLOR: u32 = 0xCCCCCCFF;
pub const MUSCLE_COLOR: u32 = CARCASS_COLOR;

pub const ZERO_BRIGHTNESS: f64 = 0.8;
pub const BRIGHTNESS_RANGE: f64 = 0.2;

pub const RELATIVE_CONF_PATH: [&str; 2] = ["config", "main.yaml"];

// this light vector must be normalized and it must direct to light source
pub const LIGHT_SOURCE_DIRECTION: Vec3d = Vec3d {
    x: 0.57735,
    y: -0.57735,
    z: -0.57735,
};

pub mod keys {
    pub const H: u16 = 43;
    pub const L: u16 = 46;
    pub const J: u16 = 44;
    pub const K: u16 = 45;
    pub const F: u16 = 41;
    pub const T: u16 = 28;

    pub const A: u16 = 38;
    pub const S: u16 = 39;
    pub const D: u16 = 40;
    pub const W: u16 = 25;
    pub const Q: u16 = 24;
    pub const E: u16 = 26;

    pub const P: u16 = 33;
    pub const M: u16 = 58;

    pub const X: u16 = 53;
    pub const V: u16 = 55;
}
