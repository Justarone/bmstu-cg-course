use gdk_pixbuf::Pixbuf;
use super::constants;

mod muscle;
use muscle::Muscle;

#[derive(Clone)]
pub struct Controller {
    height: usize,
    width: usize,
    pb: Pixbuf,
}

impl Controller {
    pub fn new(pb: Pixbuf) -> Self {
        Self {
            height: constants::HEIGHT,
            width: constants::WIDTH,
            pb,
        }
    }

    pub fn process_key(&mut self, key: &gdk::EventKey) {
        println!("{}", key.get_hardware_keycode());
    }
}

