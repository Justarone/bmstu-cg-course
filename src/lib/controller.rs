use std::sync::{Arc, Mutex};
use termion::{color, style};
use std::time::Instant;
use log::debug;

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

    pub fn restruct_muscle(&mut self, mo: MuscleOperation) -> Result<(), String> {
        let mut muscle = self.muscle.lock().unwrap();
        muscle.restruct(mo)?;
        self.cached_muscle = Some(muscle.get_points_and_normals());
        Ok(())
    }

    pub fn get_node(&self, pos: usize) -> Result<(f64, f64), String> {
        self.muscle.lock().unwrap().get_node(pos)
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
            debug!("{}=================== DRAW ROUTINES ======================", color::Fg(color::Yellow));
            let time = Instant::now();
            clear_buffers();
            debug!("Clear buffers: {} ms", time.elapsed().as_millis());
            let time = Instant::now();
            transform_and_add(
                self.cached_muscle.as_ref().unwrap(),
                &self.matrix,
                constants::LIGHT_SOURCE_DIRECTION,
                constants::MUSCLE_COLOR,
            );
            debug!("Transform and add muscle: {} ms", time.elapsed().as_millis());
            let time = Instant::now();
            transform_and_add(
                self.cached_carcass.as_ref().unwrap(),
                &self.matrix,
                constants::LIGHT_SOURCE_DIRECTION,
                constants::CARCASS_COLOR,
            );
            debug!("Transform and add carcass: {} ms", time.elapsed().as_millis());

            let time = Instant::now();
            flush(self.pb.clone());
            debug!("Flush: {} ms", time.elapsed().as_millis());
            debug!("{}========================================================", color::Fg(color::Yellow));
        }
    }

    fn update_matrix(&mut self, operation: Operation, val: f64) {
        match operation {
            Operation::Scale => self.matrix.scale_center(val),
            Operation::Rotate(axis) => self.matrix.rotate_center(val, axis),
            Operation::Move(axis) => self.matrix.mov(val, axis),
        }
    }

    pub fn process_key(&mut self, key: u16) {
        let time = Instant::now();
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
                debug!("{}MATRIX UPDATE TIME: {} ms", color::Fg(color::Magenta), time.elapsed().as_millis());
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
                debug!("{}DEFORM TIME: {} ms", color::Fg(color::LightMagenta), time.elapsed().as_millis());
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
        debug!("{}{}TOTAL PROCESSING TIME: {} ms", color::Fg(color::Red), style::Bold, time.elapsed().as_millis());
    }
}
