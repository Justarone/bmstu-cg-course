mod ui;
mod controller;
mod muscle;
mod primitives;
mod utils;
mod cg;
pub mod constants;

pub mod prelude {
    pub use super::ui::{ build_ui, process_key };
    pub use super::controller::Controller;
    pub use super::muscle::Muscle;
    pub use super::primitives::{ Polygon, Point3d, IntYPoint3d, Vec2d, Vec3d, Matrix4,
                                 Transformator, Axis, Section };
    pub use super::constants;
    pub use super::utils::{ solve_quad_eq };
    pub use super::constants::keys;
    pub use super::cg::{ transform_and_flush, clear_buffers };
    pub use gdk_pixbuf::Pixbuf;
}
