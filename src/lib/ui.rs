use gdk::prelude::*;
use gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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

pub fn process_key(controller: &Arc<Mutex<Controller>>, drawing_area: &gtk::DrawingArea, key: u16) {
    {
        let mut contr = controller.lock().unwrap();
        contr.process_key(key);
    }

    drawing_area.queue_draw();
}

pub fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Muscle");

    let glade_src = "config/control_window.glade";
    let builder = gtk::Builder::from_file(glade_src);
    let control_window: gtk::Window = builder.get_object("window").expect("Couldn't get window");
    control_window.set_application(Some(app));
    control_window.set_title("Control panel");

    let fixed = gtk::Fixed::new();
    let drawing_area = gtk::DrawingArea::new();
    fixed.add(&drawing_area);
    window.add(&fixed);
    drawing_area.set_size_request(constants::WIDTH as i32, constants::HEIGHT as i32);

    let Config {
        muscle_config: mconf,
        carcass_config: cconf,
    } = read_from_config();
    let muscle = Arc::new(Mutex::new(Muscle::new(
        mconf.radiuses,
        mconf.grow_mults,
        mconf.len,
    )));
    let carcass = Arc::new(Mutex::new(Carcass::new(
        cconf.data,
        cconf.thickness,
        mconf.len,
    )));
    let pixbuf = Pixbuf::new(
        Colorspace::Rgb,
        constants::HAS_ALPHA,
        constants::BITS_PER_COLOR,
        constants::WIDTH as i32,
        constants::HEIGHT as i32,
    )
    .unwrap();

    let mut controller = Controller::new(pixbuf.clone(), muscle, carcass);
    controller.update_pixbuf();
    let controller = Arc::new(Mutex::new(controller));

    drawing_area.connect_draw(clone!(pixbuf => move |_, context| {
        context.set_source_pixbuf(&pixbuf, 0_f64, 0_f64);
        context.paint();
        Inhibit(false)
    }));

    window.connect_key_press_event(clone!(controller, drawing_area => move |_, key| {
        process_key(&controller, &drawing_area, key.get_hardware_keycode());
        Inhibit(false)
    }));

    window.show_all();
    setup_control_window(&builder, &controller, &drawing_area);
    control_window.show_all();
}

fn setup_control_window(
    builder: &gtk::Builder,
    controller: &Arc<Mutex<Controller>>,
    drawing_area: &gtk::DrawingArea,
) {
    for (btn_name, key) in constants::COMMANDS_BUTTONS
        .iter()
        .zip(constants::CMDS_BTNS_KEY_MAP.iter())
    {
        let btn: gtk::Button = builder
            .get_object(btn_name)
            .expect(&format!("get {} object", btn_name));
        btn.connect_clicked(clone!(controller, drawing_area, key => move |_| {
            process_key(&controller, &drawing_area, key);
        }));
    }
    let mut inputs: Vec<gtk::Entry> = Vec::with_capacity(constants::INPUTS_AMOUNT);
    for inp_name in constants::INPUTS_NAMES.iter() {
        inputs.push(builder.get_object(inp_name).unwrap());
    }
    let mut rbtns: Vec<gtk::Button> = Vec::with_capacity(constants::RBTNS_AMOUNT);
    for rbtn_name in constants::RBTNS_NAMES.iter() {
        rbtns.push(builder.get_object(rbtn_name).unwrap());
    }

    rbtns[constants::DEL_BTN].connect_clicked(
        clone!(inputs, controller, drawing_area => move |_| {
            let pos = match parse_or_show_err(inputs[constants::POS_INPUT].get_buffer().get_text()) {
                Ok(val) => val,
                Err(_) => return,
            };

            {
                let mut controller = controller.lock().unwrap();
                if let Err(text) = controller.restruct_muscle(MuscleOperation::Del(pos)) {
                    show_error(text.to_string());
                    return;
                }
                controller.update_pixbuf();
            }

            drawing_area.queue_draw();
        }),
    );

    rbtns[constants::MOD_BTN].connect_clicked(
        clone!(inputs, controller, drawing_area => move |_| {
            let (pos, rad, gm) = match parse_all(&inputs) {
                Ok(res) => res,
                Err(_) => return,
            };
            {
                let mut controller = controller.lock().unwrap();
                if let Err(text) = 
                    controller.restruct_muscle(MuscleOperation::Mod(MOParams::new(pos, rad, gm))) {
                    show_error(text.to_string());
                }
                controller.update_pixbuf();
            }

            drawing_area.queue_draw();
        }),
    );

    rbtns[constants::ADD_BTN].connect_clicked(
        clone!(inputs, controller, drawing_area => move |_| {
            let (pos, rad, gm) = match parse_all(&inputs) {
                Ok(res) => res,
                Err(_) => return,
            };
            {
                let mut controller = controller.lock().unwrap();
                if let Err(text) = 
                    controller.restruct_muscle(MuscleOperation::Add(MOParams::new(pos, rad, gm))) {
                    show_error(text.to_string());
                }
                controller.update_pixbuf();
            }

            drawing_area.queue_draw();
        }),
    );
}

fn parse_all(inputs: &Vec<gtk::Entry>) -> Result<(usize, f64, f64), ()> {
    let pos = match parse_or_show_err(inputs[constants::POS_INPUT].get_buffer().get_text()) {
        Ok(val) => val,
        Err(_) => return Err(()),
    };
    let rad = match parse_or_show_err::<f64>(inputs[constants::RAD_INPUT].get_buffer().get_text()) {
        Ok(val) => val,
        Err(_) => return Err(()),
    };
    let gm = match parse_or_show_err::<f64>(inputs[constants::GM_INPUT].get_buffer().get_text()) {
        Ok(val) => val,
        Err(_) => return Err(()),
    };
    Ok((pos, rad, gm))
}

fn parse_or_show_err<T: std::str::FromStr>(text: String) -> Result<T, ()> {
    match text.parse::<T>() {
        Ok(val) => Ok(val),
        Err(_) => {
            show_error(format!("Parse error: {}", text));
            Err(())
        }
    }
}

fn show_error(text: String) {
    let dialog = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::empty(),
        gtk::MessageType::Error,
        gtk::ButtonsType::None,
        &text,
    );
    dialog.set_title("Error");
    dialog.run();
}
