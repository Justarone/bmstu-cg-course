use gdk_pixbuf::Pixbuf;
use std::sync::{ Arc, Mutex };

use super::prelude::*;
use keys::*;

#[derive(Clone)]
pub struct Controller {
    height: usize,
    width: usize,
    pb: Pixbuf,
    muscle: Arc<Mutex<Muscle>>,
    matrix: Matrix4,
    cached_muscle: (Vec<Point3d>, Vec<Vec3d>),
}

impl Controller {
    pub fn new(pb: Pixbuf, muscle: Arc<Mutex<Muscle>>) -> Self {
        Self {
            height: constants::HEIGHT,
            width: constants::WIDTH,
            pb,
            muscle,
            matrix: Matrix4::identity(),
        }
    }

    fn update_muscle(&mut self, diff: f64) {
        let mut muscle = self.muscle.lock().unwrap();
        muscle.deform(diff);
    }

    pub fn process_key(&mut self, key: &gdk::EventKey) {
        match key.get_hardware_keycode() {
            H => {
                println!("Rotate left: H({})", H);
                // update_matrix
                // update_pixbuf
            }
            L => {
                println!("Rotate right: L({})", L);
                // update_matrix
                // update_pixbuf
            }
            J => {
                println!("Rotate down: J({})", J);
            }
            K => {
                println!("Rotate up: K({})", K);
            }
            F => {
                println!("Rotate clockwise: F({})", F);
            }
            T => {
                println!("Rotate counterclock-wise: T({})", T);
            }

            A => {
                println!("Move left: A({})", A);
            }
            S => {
                println!("Move down: S({})", S);
            }
            D => {
                println!("Move right: D({})", D);
            }
            W => {
                println!("Move up: W({})", W);
            }
            Q => {
                println!("Move top: Q({})", Q);
            }
            E => {
                println!("Move bottom: E({})", E);
            }

            P => {
                println!("Scale plus: P({})", P);
            }
            M => {
                println!("Scale minus: M({})", M);
            }

            X => {
                println!("Shorten: X({})", X);
            }
            V => {
                println!("Lengthen: V({})", V);
            }

            val => println!("Unknown command: {}", val),
        }
    }
}

