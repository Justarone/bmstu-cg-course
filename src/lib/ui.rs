use gtk::prelude::*;
use gdk::prelude::*;
use gdk_pixbuf::{ Colorspace, Pixbuf };
use std::sync::{ Mutex, Arc };

use super::prelude::*;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


pub fn process_key(controller: &Arc<Mutex<Controller>>, drawing_area: &gtk::DrawingArea, key: &gdk::EventKey) {
    {
        let mut contr = controller.lock().unwrap();
        contr.process_key(key);
    }

    drawing_area.queue_draw();
}

pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Muscle");

    let fixed = gtk::Fixed::new();
    let drawing_area = gtk::DrawingArea::new();
    fixed.add(&drawing_area);
    window.add(&fixed);
    drawing_area.set_size_request(constants::WIDTH as i32, constants::HEIGHT as i32);

    let Config { radiuses: rads, grow_mults: gm, len } = read_from_config();
    let muscle = Arc::new(Mutex::new(Muscle::new(rads, gm, len)));
    let pixbuf = Pixbuf::new(Colorspace::Rgb, constants::HAS_ALPHA, constants::BITS_PER_COLOR,
        constants::WIDTH as i32, constants::HEIGHT as i32).unwrap();

    let mut controller = Controller::new(pixbuf.clone(), muscle);
    controller.update_pixbuf();
    let controller = Arc::new(Mutex::new(controller));


    drawing_area.connect_draw(
        clone!(pixbuf => move |_, context| {
            context.set_source_pixbuf(&pixbuf, 0_f64, 0_f64);
            context.paint();
            Inhibit(false)
    }));

    
    window.connect_key_press_event(
        clone!(controller, drawing_area => move |_, key| {
            process_key(&controller, &drawing_area, key);
            Inhibit(false)
    }));

    window.show_all();
}
