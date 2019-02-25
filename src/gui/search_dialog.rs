extern crate gtk;

use gtk::prelude::*;
use std::thread;
use std::sync::{Arc,Mutex};
use app::EpicwarApplication;
use gui::diff_window::ReplacedWindow;
use gui::map_box::MapBox;
use wc3;
use super::*;


#[derive(Clone)]
pub struct SearchDialog {
    pop_widget: gtk::Window,
    name_entry: gtk::Entry,
    author_entry: gtk::Entry,
    search_button: gtk::Button,
}

impl SearchDialog {
    pub fn new() -> Self {
        // Create the barebones dialog.
        let window = gtk::Window::new(gtk::WindowType::Popup);
        window.set_default_size(300,100);
        window.set_title("Crawl Options");
        window.set_resizable(false);
        // This container will contain all of the settings 
        // as far as crawling goes.

        // This container will contain all of the settings
        // as far as local searching goes ( after crawling ).
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 16);
        

        // Entry for searching by map name.
        let name_entry = gtk::Entry::new();
        name_entry.set_placeholder_text(Some("Map name..."));

        // Entry for searching by author name.
        let author_entry = gtk::Entry::new();
        author_entry.set_placeholder_text(Some("Author name..."));

        let play_image = gtk::Image::new_from_file("/home/abovegame/Documents/RustProjects/epicwar-downloader/icons/play-button.png");
        let run_button = gtk::Button::new();
        run_button.set_image(Some(&play_image));

        vbox.add(&name_entry);
        vbox.add(&author_entry);
        vbox.pack_end(&run_button,false,false,0);

        window.add(&vbox);

        let pw = Self {
            pop_widget: window,
            name_entry: name_entry,
            author_entry: author_entry,
            search_button: run_button,
        };
        pw 
    }
    pub fn initialize_callbacks(rw: &ReplacedWindow,main_app: Arc<EpicwarApplication>) {
        let pw = &rw.popover_widget;
        pw.search_button.connect_clicked(clone!(pw,rw => move |b| {
            b.set_sensitive(false);
            // Check the stack switcher and determine which
            // type of search we should perform.
            let author_name = pw.author_entry.get_text().unwrap();
            let map_name = pw.name_entry.get_text().unwrap();
            // Once that is done. Either `crawl` or `search` 
            // the local map box collection.

            // Out `is_finished` stores a boolean value which
            // represents if the spawned thread has ended.
            let mut is_finished = vec![];
            for page in 0..5 {
                let current = Arc::new(Mutex::new(false));
                is_finished.push(current.clone());
                thread::spawn(clone!(main_app => move || {
                    wc3::crawl(main_app.client.clone(), main_app.map_collection.clone(), page);
                    *current.lock().unwrap() = true;
                }));
            }

            
            gtk::timeout_add(500,clone!(pw,rw,main_app => move || {
                // Checking if all the threads have ended 
                // which in turn means we can display 
                // our maps in the window.
                if is_finished.iter().filter(|e| !*e.lock().unwrap()).collect::<Vec<_>>().len() == 0 {
                    pw.pop_widget.close();
                    pw.search_button.set_sensitive(true);
                    for m in main_app.map_collection.lock().unwrap().iter() {
                        rw.container.add(&MapBox::new(m.clone()).container);
                    }
                    return gtk::Continue(false);
                }
                gtk::Continue(true)
            }));

        }));
        
        pw.pop_widget.connect_delete_event(|_,_| {
            Inhibit(true)
        });
    }

    pub fn show(&self) {
        self.pop_widget.show_all();
    }
}