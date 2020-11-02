use std::sync::{Arc, Mutex};
use termion::{color, style};

use super::prelude::*;
use keys::*;

enum Operation {
    Scale,
    Rotate(Axis),
    Move(Axis),
}

#[derive(Clone)]
pub struct Controller {
    height: usize,
    width: usize,
    pb: Pixbuf,
    muscle: Arc<Mutex<Muscle>>,
    carcass: Arc<Mutex<Carcass>>,
    matrix: Matrix4,
    cached_muscle: Option<(Vec<Vec<Point3d>>, Vec<Vec<Point3d>>)>,
    cached_carcass: Option<(Vec<Vec<Point3d>>, Vec<Vec<Point3d>>)>,
}

impl Controller {
    pub fn new(pb: Pixbuf, muscle: Arc<Mutex<Muscle>>, carcass: Arc<Mutex<Carcass>>) -> Self {
        let mut matrix = Matrix4::identity();
        matrix.mov((constants::WIDTH / 2) as f64, Axis::X);
        matrix.mov((constants::HEIGHT / 2) as f64, Axis::Y);

        Self {
            height: constants::HEIGHT,
            width: constants::WIDTH,
            pb,
            carcass,
            muscle,
            matrix,
            cached_muscle: None,
            cached_carcass: None,
        }
    }

    fn deform(&mut self, diff: f64) {
        let mut muscle = self.muscle.lock().unwrap();
        let mut carcass = self.carcass.lock().unwrap();
        if carcass.check_diff(diff) {
            carcass.deform(diff);
            muscle.deform(diff);
            self.cached_muscle = Some(muscle.get_points_and_normals());
            self.cached_carcass = Some(carcass.get_points_and_normals());
        }
    }

    pub fn update_pixbuf(&mut self) {
        if let None = self.cached_muscle {
            let muscle = self.muscle.lock().unwrap();
            self.cached_muscle = Some(muscle.get_points_and_normals());
        }

        if let None = self.cached_carcass {
            let carcass = self.carcass.lock().unwrap();
            self.cached_carcass = Some(carcass.get_points_and_normals());
        }

        unsafe {
            clear_buffers();
            transform_and_flush(
                self.cached_muscle.as_ref().unwrap(),
                &self.matrix,
                self.pb.clone(),
                constants::LIGHT_SOURCE_DIRECTION,
                constants::MUSCLE_COLOR,
            );
            transform_and_flush(
                self.cached_carcass.as_ref().unwrap(),
                &self.matrix,
                self.pb.clone(),
                constants::LIGHT_SOURCE_DIRECTION,
                constants::CARCASS_COLOR,
            );
        }
    }

    fn update_matrix(&mut self, operation: Operation, val: f64) {
        match operation {
            Operation::Scale => self.matrix.scale_center(val),
            Operation::Rotate(axis) => self.matrix.rotate_center(val, axis),
            Operation::Move(axis) => self.matrix.mov(val, axis),
        }
    }

    pub fn process_key(&mut self, key: &gdk::EventKey) {
        let key = key.get_hardware_keycode();
        match key {
            // operations only with transformation matrix
            H | L | J | K | F | T | A | S | D | W | Q | E | P | M => {
                let (operation, val) = match key {
                    H => (Operation::Rotate(Axis::Y), constants::ROTATE_VAL),
                    L => (Operation::Rotate(Axis::Y), -constants::ROTATE_VAL),
                    J => (Operation::Rotate(Axis::X), constants::ROTATE_VAL),
                    K => (Operation::Rotate(Axis::X), -constants::ROTATE_VAL),
                    F => (Operation::Rotate(Axis::Z), constants::ROTATE_VAL),
                    T => (Operation::Rotate(Axis::Z), -constants::ROTATE_VAL),

                    A => (Operation::Move(Axis::X), -constants::MOVE_VAL),
                    D => (Operation::Move(Axis::X), constants::MOVE_VAL),
                    S => (Operation::Move(Axis::Y), constants::MOVE_VAL),
                    W => (Operation::Move(Axis::Y), -constants::MOVE_VAL),
                    Q => (Operation::Move(Axis::Z), constants::MOVE_VAL),
                    E => (Operation::Move(Axis::Z), -constants::MOVE_VAL),

                    P => (Operation::Scale, constants::SCALE_VAL),
                    M => (Operation::Scale, 1_f64 / constants::SCALE_VAL),
                    _ => unreachable!("No way"),
                };

                self.update_matrix(operation, val);
                self.update_pixbuf();
            }

            // operations with muscles
            X | V => {
                let diff = match key {
                    X => -constants::ATOM_DIFF,
                    V => constants::ATOM_DIFF,
                    _ => unreachable!("No way"),
                };

                self.deform(diff);
                self.update_pixbuf();
            }

            // unknown keys
            val => println!(
                "{}Unknown command: {}{}",
                color::Fg(color::Red),
                val,
                style::Reset
            ),
        }
    }
}
