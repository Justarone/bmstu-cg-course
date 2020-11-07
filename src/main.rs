#[macro_use]
extern crate approx;

use gio::prelude::*;

mod lib;
use lib::prelude::build_ui;

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .init();
    let application =
        gtk::Application::new(None, Default::default()).expect("Init GTK application");
    application.connect_activate(build_ui);
    application.run(&std::env::args().collect::<Vec<_>>());
}
