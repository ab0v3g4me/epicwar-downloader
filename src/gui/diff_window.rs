use gtk::prelude::*;

use gui::*;
use wc3;
use app::EpicwarApplication;
use std::sync::{Arc};

#[derive(Clone)]
pub struct ReplacedWindow {
   pub window: gtk::Window,
   pub container: gtk::Box,
   pub status_bar: gtk::Statusbar,
   pub popover_widget: SearchDialog,
   pub search_button: gtk::Button,
}
impl ReplacedWindow {
    pub fn new(main_app: Arc<EpicwarApplication>) -> Arc<EpicwarApplication> {
        // Creating our window object.
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        // Creating our `ScrolledWindow` widget which we will later add as
        // a main container to our window.
        let sw = gtk::ScrolledWindow::new(None,None);
        sw.set_policy(gtk::PolicyType::Never,gtk::PolicyType::Automatic);
        sw.set_border_width(0);

        // Add a button that runs the `SearchDialog`;
        let hb_search_button = gtk::Button::new();

        // Creating our `HeaderBar` widget. Setting the title and 
        // showing the close button.
        let hb = gtk::HeaderBar::new();
        hb.pack_start(&hb_search_button);
        hb.set_title("Epicwar Downloader");
        hb.set_show_close_button(true);


        let sd = SearchDialog::new(); 
        // Add a status bar for page number indication.
        let status_bar = gtk::Statusbar::new();
        status_bar.push(0, "Press ALT key for the crawl menu.");
        // Horizontal box that will contain all dynamically 
        // made map boxes.        
        let hbox = gtk::Box::new(gtk::Orientation::Vertical,0);

        hbox.pack_end(&status_bar,false,false,0);
        sw.add(&hbox);

        window.add(&sw);
        window.set_titlebar(&hb);
        window.set_size_request(900,600);
        window.set_resizable(false);
        window.show_all();
        
        let rw = Self {
            window: window,
            container: hbox,
            status_bar: status_bar,
            popover_widget: sd,
            search_button: hb_search_button,
        };

        SearchDialog::initialize_callbacks(&rw, main_app.clone());
        Self::initialize_callbacks(&rw);

        main_app
    }
    fn initialize_callbacks(rw: &Self) {
        rw.search_button.connect_clicked(clone!(rw => move |_|{
            rw.popover_widget.show();
        }));
        rw.window.connect_delete_event(|_,_|{
            gtk::main_quit();
            Inhibit(false)
        });
    }
   
}