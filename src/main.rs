extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate cairo;
#[macro_use]
extern crate approx;

use gio::prelude::*;

mod lib;
use lib::prelude::build_ui;

fn main() {
    let application = gtk::Application::new(None, Default::default())
        .expect("failed to initialize GTK application");
    application.connect_activate(build_ui);
    application.run(&std::env::args().collect::<Vec<_>>());
}
