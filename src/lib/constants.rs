pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const BITS_PER_COLOR: i32 = 8;
pub const HAS_ALPHA: bool = true;

pub const MIN_PART: f64 = 0.4;
pub const MAX_PART: f64 = 2.5;

pub const DEGREES: usize = 360;
pub const STEP: usize = 2;
pub const SPHERE_PARTS: usize = 10;

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
