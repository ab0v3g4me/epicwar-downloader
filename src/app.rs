extern crate gtk;
extern crate reqwest;
extern crate select;
extern crate gdk_pixbuf;

use gtk::prelude::*;
use wc3::*;
use std::rc::Rc;
use std::sync::{Arc,Mutex};
use std::collections::HashMap;
use gui::*;

pub struct EpicwarApplication {
    pub map_collection: Arc<Mutex<Vec<WC3Map>>>,
    pub client: Arc<reqwest::Client>,
}

impl EpicwarApplication {
    pub fn run() -> Arc<Self> {
        let ea = EpicwarApplication {
            map_collection: Arc::new(Mutex::new(Vec::new())),
            client: Arc::new(reqwest::Client::new()),
        };
        ReplacedWindow::new(Arc::new(ea))
    }

}