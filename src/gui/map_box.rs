extern crate gtk;

use gtk::prelude::*;
use std::cell::RefCell;
use std::thread;
use wc3;
use std::sync::{Arc,Mutex};
use std::sync::mpsc::channel;

#[macro_use(clone)]
use super::*;

#[derive(Clone,PartialEq)]
pub enum ButtonState {
    Idle,
    Downloading,
    Finished,
    Failed,
}

#[derive(Clone)]
pub struct MapBox {
    pub container: gtk::Box,
    pub button: gtk::Button,
    pub state: Arc<Mutex<ButtonState>>,
    pub map: Arc<wc3::WC3Map>,
}

impl MapBox {
    pub fn new(map: wc3::WC3Map) -> Self {
        // This gtk::Box object will contain all of the map info
        // including the progress bar and download button. 
        let map_box = gtk::Box::new(gtk::Orientation::Horizontal,8);

        // Creating a map name label.
        let map_name = gtk::Label::new(Some(&map.get_map_info()[..]));
        // Creating an image icon for number of players.
        let img_name = format!("/home/abovegame/Documents/RustProjects/epicwar-downloader/icons/{}.png", map.max_players);
        let max_players_img = gtk::Image::new_from_file(&img_name[..]);
        
        // Creating the download button.
        let download_image = gtk::Image::new_from_file("/home/abovegame/Documents/RustProjects/epicwar-downloader/icons/download-button.png");
        let download_button = gtk::Button::new();
        download_button.set_always_show_image(true);
        download_button.set_image(Some(&download_image));

        //let img_pixbuff = if let Ok(pixbuff) = gdk_pixbuf::Pixbuf::new_from_resource(&map.get_icon_path()[..]).unwrap();
        //img_pixbuff.scale_simple(16,16, gdk_pixbuf::InterpType::Bilinear);
        map_box.set_border_width(8);
        map_box.pack_start(&max_players_img,false,false,8);
        map_box.pack_start(&map_name,false,false,8);
        map_box.pack_end(&download_button,false,false,8);
        
        let mb = MapBox {
            container: map_box,
            button: download_button,
            state: Arc::new(Mutex::new(ButtonState::Idle)),
            map: Arc::new(map),
        };
        mb.container.show_all();
        Self::initialize_callbacks(&mb);
        mb
    }
    fn initialize_callbacks(mb: &Self) {
        
        mb.button.connect_clicked(clone!(mb => move |b|{
            // Checking if the button wasn't clicked.
            // If so change the icon to the cancel icon.
            let state = if let Ok(value) = mb.state.lock() {
                value.clone()
            } else {
                return;
            };
            match state {
                ButtonState::Idle => {
                    let cancel_image = gtk::Image::new_from_file("/home/abovegame/Documents/RustProjects/epicwar-downloader/icons/cancel-button.png");
                    b.set_image(Some(&cancel_image));
                    *mb.state.lock().unwrap() = ButtonState::Downloading;

                    let map_handle = mb.map.clone();
                    let state_handle = mb.state.clone();
                    thread::spawn(move || {
                        wc3::WC3Map::download(map_handle,state_handle);
                    });
                },
                // Handles the canceling of a download.
                ButtonState::Downloading => {
                    *mb.state.lock().unwrap() = ButtonState::Idle;
                    let download_image = gtk::Image::new_from_file("/home/abovegame/Documents/RustProjects/epicwar-downloader/icons/download-button.png");
                    b.set_image(Some(&download_image));
                },
                _ => (),
            };
            // Checking if the state is `Finished` in order
            // to change the button icon.
            gtk::timeout_add(1000,clone!(mb => move ||{
                if let Ok(state) = mb.state.lock() {
                    match *state {
                        ButtonState::Finished => {
                            let finish_image = gtk::Image::new_from_file("/home/abovegame/Documents/RustProjects/epicwar-downloader/icons/finished-button.png");
                            mb.button.set_image(Some(&finish_image));
                            mb.button.set_sensitive(false);
                            return gtk::Continue(false);
                        },
                        ButtonState::Failed => {
                            return gtk::Continue(false);
                        },
                        _ => (),
                    }
                }
                gtk::Continue(true)
            }));
        }));
        mb.button.connect_enter_notify_event(clone!(mb => move |b,event| {
            Inhibit(false)
        }));
        mb.button.connect_leave_notify_event(clone!(mb => move |b,event| {
            Inhibit(false)
        }));

    }
}
