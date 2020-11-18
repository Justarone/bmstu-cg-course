mod carcass;
mod cg;
pub mod constants;
mod controller;
mod muscle;
mod primitives;
mod stubs;
mod ui;
mod utils;

pub mod prelude {
    pub use super::cg::{clear_buffers, flush, transform_and_add};
    pub use super::constants;
    pub use super::constants::keys;
    pub use super::controller::Controller;
    pub use super::muscle::{MOParams, Muscle, MuscleOperation};
    pub use super::primitives::{
        Axis, CenterTransformator, IntYPoint3d, Matrix4, Point3d, Section, Transformator, Vec2d,
        Vec3d,
    };
    pub use super::ui::{build_ui, process_key};
    pub use super::utils::{
        add_uv_sphere, angle_from_triangle, cycle_extend, read_from_config, rotate_intersections,
        solve_quad_eq, CarcassConfig, Config, MuscleConfig,
    };
    //pub use super::stubs::{ dy_stub };
    pub use super::carcass::Carcass;
    pub use gdk_pixbuf::Pixbuf;
}
