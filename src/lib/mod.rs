mod ui;
mod controller;
mod muscle;
mod carcass;
mod primitives;
mod utils;
mod cg;
mod stubs;
pub mod constants;

pub mod prelude {
    pub use super::ui::{ build_ui, process_key };
    pub use super::controller::Controller;
    pub use super::muscle::Muscle;
    pub use super::primitives::{ Transformator, CenterTransformator, Point3d, IntYPoint3d,
        Vec2d, Vec3d, Matrix4, Axis, Section };
    pub use super::constants;
    pub use super::utils::{ solve_quad_eq, MuscleConfig, CarcassConfig, Config, read_from_config,
        cycle_extend, add_uv_sphere, rotate_intersections, angle_from_triangle };
    pub use super::constants::keys;
    pub use super::cg::{ transform_and_flush, clear_buffers };
    //pub use super::stubs::{ dy_stub };
    pub use super::carcass::Carcass;

    pub use gdk_pixbuf::Pixbuf;
}
