extern crate gtk;
extern crate reqwest;
extern crate select;

use gtk::prelude::*;
use std::sync::{Arc,Mutex};
use std::thread;
use std::sync::mpsc::{sync_channel};
use select::predicate::{Not,Class, Name,Attr};
use wc3::*;
use std::env;

pub struct  App{
    w: gtk::Window,
    map_container: gtk::Box,
    pub wc3db: Arc<Mutex<Vec<WC3Map>>>,
    css_style: gtk::CssProvider,
    pb_collection: Arc<Mutex<Vec<gtk::ProgressBar>>>,
}

impl  App {

    pub fn new() -> Self {
        App {
            w: gtk::Window::new(gtk::WindowType::Toplevel),
            map_container: gtk::Box::new(gtk::Orientation::Vertical,20),
            wc3db: Arc::new(Mutex::new(Vec::new())),
            css_style: gtk::CssProvider::new(),
            pb_collection: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn run(&self){
        let hb = gtk::HeaderBar::new();
        hb.set_title("Epicwar Downloader");
        hb.set_show_close_button(true);
        let next_button = gtk::Button::new_with_label("Next Page");
        let prev_button = gtk::Button::new_with_label("Previous Page");

        hb.pack_end(&prev_button);
        hb.pack_end(&next_button);

        let crawl_button = gtk::Button::new_with_label("Crawl");

        hb.pack_start(&crawl_button);

        let spin_adj = gtk::Adjustment::new(1.0,1.0,100.0,1.0,1.0,0.0);
        let spin_button = gtk::SpinButton::new(&spin_adj,1.0,0);

        hb.pack_start(&spin_button);

        let sw = gtk::ScrolledWindow::new(None,None);
        sw.set_policy(gtk::PolicyType::Never,gtk::PolicyType::Automatic);
        sw.set_border_width(5);

        sw.add(&self.map_container);

        self.w.add(&sw);
        self.w.set_titlebar(&hb);
        self.w.set_size_request(900,600);
        self.w.set_resizable(false);
        self.w.show_all();


        let wc3db_clone = self.wc3db.clone();
        let sb = spin_button.clone();
        let css_style =self.css_style.clone();
        let mc = self.map_container.clone();
        let pb_coll = self.pb_collection.clone();

        let current_page = Arc::new(Mutex::new(1));
        let max_pages = Arc::new(Mutex::new(0));
        let max_pages_cloned = max_pages.clone();


        crawl_button.connect_clicked(move |b|{

            b.set_sensitive(false);

            let pages = sb.get_value() as usize;
            *max_pages_cloned.lock().unwrap() = pages;

            let wc3db_clone = wc3db_clone.clone();
            let wc3db_clone_1 = wc3db_clone.clone();

            let pb_coll = pb_coll.clone();
            pb_coll.lock().unwrap().clear();
            let mc = mc.clone();
            App::clear_box(&mc);

            thread::spawn(move ||{
                let c = reqwest::Client::new();
                wc3db_clone.lock().unwrap().clear();
                for i in 0..pages {
                App::crawl(&c ,&wc3db_clone, i + 1);
                        }
            });

            let b = b.clone();

            let css_style = css_style.clone();
            gtk::timeout_add(1000,move  ||{
                    let pb_coll = pb_coll.clone();
                    let css_style = css_style.clone();
                    if wc3db_clone_1.lock().unwrap().len() > 0 {
                        App::fill_mapbox(css_style, &wc3db_clone_1,&mc,1,&pb_coll);
                        b.set_sensitive(true);
                        return gtk::Continue(false);
                    }
                    gtk::Continue(true)
            });
        });

        let wc3db_clone = self.wc3db.clone();
        let mc = self.map_container.clone();
        let css_style = self.css_style.clone();
        let current_page_1 = current_page.clone();
        let max_pages_cloned = max_pages.clone();
        let pb_coll = self.pb_collection.clone();
        next_button.connect_clicked(move |_| {
            let pb_coll = pb_coll.clone();
            let mut c = current_page_1.lock().unwrap();
            let css_style = css_style.clone();
            if *c < *max_pages_cloned.lock().unwrap() {
                *c += 1;
                App::clear_box(&mc);
                App::fill_mapbox(css_style,&wc3db_clone,&mc,*c,&pb_coll);
            }
        });

        let wc3db_clone = self.wc3db.clone();
        let mc = self.map_container.clone();
        let css_style = self.css_style.clone();
        let pb_coll = self.pb_collection.clone();
        let current_page_1 = current_page.clone();
        prev_button.connect_clicked(move |_|{
            let pb_coll = pb_coll.clone();
            let css_style = css_style.clone();
            let mut c = current_page_1.lock().unwrap();
            if *c > 1 {
                *c -= 1;
                App::clear_box(&mc);
                App::fill_mapbox(css_style, &wc3db_clone,&mc,*c,&pb_coll);
            }
        });
        self.w.connect_delete_event(|_,_|{
            gtk::main_quit();

            Inhibit(false)
        });

    }

    pub fn fill_mapbox(css_style: gtk::CssProvider,wc3db: &Arc<Mutex<Vec<WC3Map>>>,map_container: &gtk::Box,page:usize,pb_coll: &Arc<Mutex<Vec<gtk::ProgressBar>>>) {
        let p = String::from(env::current_exe().unwrap().parent().unwrap().to_str().unwrap());
        let p = format!("{}/button.css",p);
        css_style.load_from_path(&p[..]).unwrap();
        let wc3db_1 = wc3db.lock().unwrap();
        let mut pb_coll = pb_coll.lock().unwrap();
        let it = (0 + (25*( page - 1 ) ))..(25 + (25* ( page - 1 ) ));
        for i in it {
            let m = &wc3db_1[i];
            let bx = gtk::Box::new(gtk::Orientation::Horizontal,0);

            let l = gtk::Label::new(Some(&m.get_map_info()[..]));
            let l_style = l.get_style_context().unwrap();
            l_style.add_provider(&css_style,gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

            let img = gtk::Image::new_from_file(&m.get_icon_path()[..]);

            let cond = m.dl_started.clone();

            let dl_finished = m.dl_finished.clone();
            let label = if *dl_finished.lock().unwrap() == true {
                "Finished"
            } else {
                "Download"
            };

            let bt = gtk::Button::new_with_label(label);
            let bt_style = bt.get_style_context().unwrap();
            bt_style.add_provider(&css_style,gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            if *cond.lock().unwrap() == true { bt.set_sensitive(false); }
            let mut pb: gtk::ProgressBar;
            if pb_coll.len() < i + 1 {
                pb = gtk::ProgressBar::new();
                let pb_style = pb.get_style_context().unwrap();
                pb_style.add_provider(&css_style,gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
                pb_coll.push(pb.clone());
            }
            else { pb = pb_coll[i].clone();}



            bx.pack_start(&img,false,false,8);
            bx.pack_start(&l,false,false,8);
            bx.pack_end(&bt,false,false,16);
            bx.pack_end(&pb,false,false,16);

            map_container.add(&bx);

            let file_name = m.file_name.clone();
            let link = m.map_link.clone();

            let pb_clone = pb.clone();
            let cond = cond.clone();
            let dl_finished = m.dl_finished.clone();
            bt.connect_clicked(move |b|{
                *cond.lock().unwrap() = true;
                b.set_sensitive(false);


                let file_name = file_name.clone();
                let link = link.clone();
                let (tx,rx)= sync_channel(1);

                thread::spawn(move ||{

                        match WC3Map::download(file_name,link,tx){
                            Ok(_) => (),
                            Err(_) => (),
                         };
                });

                let pb_clone = pb_clone.clone();
                let b = b.clone();
                let dl_finished = dl_finished.clone();
                gtk::timeout_add(500,move ||{
                    let prog = match rx.try_recv(){
                        Ok(v) => v,
                        Err(_) => 0.0,
                    };

                    pb_clone.set_fraction(prog);
                    if  prog == 1.0 {
                        println!("Finished");
                        *dl_finished.lock().unwrap() = true;
                        b.set_label("Finished");
                        return gtk::Continue(false); }
                    gtk::Continue(true)
                });

            });
        }

        map_container.show_all();
    }

    pub fn crawl(c: &reqwest::Client,wc3db: & Arc<Mutex<Vec<WC3Map>>>,i: usize) {

        let url = format!("https://www.epicwar.com/maps/?page={}",i);

        let mut r = c.get(&url[..]).send().unwrap();

        let html = r.text().unwrap();
        let d = select::document::Document::from(html.as_str());
        let table = d.find(Name("tbody")).nth(3).unwrap();

        for tr in table.find(Not(Attr("bgcolor","#333333"))){
            let class = match tr.find(Class("listentry")).nth(1) {
                Some(c) => c,
                None => continue,
            };
            let map_name = class.find(Name("b")).nth(0).unwrap().text();
            let author = class.find(Name("b")).nth(1).unwrap().text();
            let max_players = class.find(Name("img")).nth(0).unwrap().attr("alt").unwrap().to_owned();
            let map_link = class.find(Name("a")).nth(1).unwrap().attr("href").unwrap().to_owned();
            let file_name = class.find(Name("a")).nth(1).unwrap().text();

            let mut m =  WC3Map::new(author, map_name, max_players,map_link,file_name,Arc::new(Mutex::new(false)),Arc::new(Mutex::new(false)));
            wc3db.lock().unwrap().push(m);
        }
    }
    pub fn clear_box(mc: &gtk::Box) {
        for c in mc.get_children() {
            mc.remove(&c);
        }
    }
}
