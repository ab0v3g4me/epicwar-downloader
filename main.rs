extern crate reqwest;
extern crate select;
extern crate gtk;

use std::fs;
use std::env;

mod gui;
mod wc3;

fn main() {

    match gtk::init() {
        Ok(_) => (),
        Err(_) => return,
    }
    let p = String::from(env::current_exe().unwrap().parent().unwrap().to_str().unwrap());
    let p = format!("{}/maps",p);
    match fs::create_dir(p){
        Ok(_) => (),
        Err(_) => (),
    };
    let a = gui::App::new();
    a.run();
    gtk::main();

}
