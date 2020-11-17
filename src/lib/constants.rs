use super::prelude::Vec3d;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const BITS_PER_COLOR: i32 = 8;
pub const HAS_ALPHA: bool = true;

pub const MIN_PART: f64 = 0.4;
pub const MAX_PART: f64 = 2.5;

pub const NEGATIVE_Z_PROJECTION: f64 = -0.2;

pub const DEGREES: usize = 360;

pub const MUSCLE_STEP: usize = 10;
pub const CARCASS_STEP: usize = 45;
pub const SPHERE_STEP: usize = 20;

pub const SPHERE_PARTS: usize = 8;

pub const ROTATE_VAL: f64 = 0.1;
pub const MOVE_VAL: f64 = 5.0;
pub const SCALE_VAL: f64 = 1.25;

pub const ATOM_DIFF: f64 = 2.0;

pub const MIN_Z: f64 = f64::MIN;
pub const DEFAULT_COLOR: u32 = 0x1E1E1EFF;
pub const MUSCLE_COLOR: u32 = 0xCC0000FF;
pub const CARCASS_COLOR: u32 = 0xCCCCCCFF;

pub const ZERO_BRIGHTNESS: f64 = 0.6;
pub const BRIGHTNESS_RANGE: f64 = 0.4;

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

pub const COMMANDS_BUTTONS_AMOUNT: usize = 16;
pub const COMMANDS_BUTTONS: [&str; COMMANDS_BUTTONS_AMOUNT] = [
    "move_left",
    "move_up",
    "move_back",
    "move_right",
    "move_down",
    "move_front",
    "rotate_left",
    "rotate_up",
    "rotate_clockwise",
    "rotate_right",
    "rotate_down",
    "rotate_unclockwise",
    "scale_up",
    "scale_down",
    "lengthen",
    "shorten",
];

pub const CMDS_BTNS_KEY_MAP: [u16; COMMANDS_BUTTONS_AMOUNT] = [
    keys::A,
    keys::W,
    keys::E,
    keys::D,
    keys::S,
    keys::Q,
    keys::H,
    keys::K,
    keys::T,
    keys::L,
    keys::J,
    keys::F,
    keys::P,
    keys::M,
    keys::V,
    keys::X,
];

pub const INPUTS_AMOUNT: usize = 3;
pub const POS_INPUT: usize = 0;
pub const RAD_INPUT: usize = 1;
pub const GM_INPUT: usize = 2;
pub const INPUTS_NAMES: [&str; INPUTS_AMOUNT] = ["pos_input", "rad_input", "gm_input"];

pub const RBTNS_AMOUNT: usize = 7;
pub const ADD_BTN: usize = 0;
pub const DEL_BTN: usize = 1;
pub const MOD_BTN: usize = 2;
pub const MODP_BTN: usize = 3;
pub const MODM_BTN: usize = 4;
pub const NEXT_BTN: usize = 5;
pub const PREV_BTN: usize = 6;
pub const RBTNS_NAMES: [&str; RBTNS_AMOUNT] = ["add_btn", "del_btn", "mod_btn", "modp_btn", "modm_btn", "next_btn", "prev_btn"];

pub const DELTA_RAD: f64 = 1.0;
