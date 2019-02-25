extern crate reqwest;
extern crate select;
extern crate gtk;
extern crate gdk_pixbuf;

mod app;
mod gui;
mod wc3;

fn main() {
    if let Ok(_) = gtk::init() {
        app::EpicwarApplication::run();
        gtk::main();
    }
}
