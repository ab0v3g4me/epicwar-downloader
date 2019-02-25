extern crate reqwest;
extern crate select;

use std::fs;
use std::env;
use std::io::{Write,Read};
use std::clone;
use std::sync::mpsc::Receiver;
use std::sync::{Arc,Mutex};
use std::str;
use select::predicate::{Not,Class, Name,Attr};
use gui::map_box::ButtonState;

#[derive(Clone)]
pub struct WC3Map {
    author: String,
    map_name: String,
    pub max_players: String,
    pub map_link: String,
    pub file_name: String,
}

impl WC3Map {
    const BASEURL: &'static str = "https://www.epicwar.com";
    pub fn new(author: String,
               map_name: String, 
               max_players: String,
               map_link: String,
               file_name: String) -> Self {
        
        WC3Map {
            author: author,
            map_name: map_name,
            max_players: max_players,
            map_link: map_link,
            file_name: file_name,
        }
    }
    pub fn download(map: Arc<Self>,state: Arc<Mutex<ButtonState>>) {
        let fullurl = format!("{}{}",Self::BASEURL, map.map_link);

        let mut response = if let Ok(r) = reqwest::get(&fullurl[..]) {
            r
        } else {
            if let Ok(mut data) = state.lock() {
                *data = ButtonState::Failed;
            }
            return;
        };

        let total_size = if let Some(length) = response.headers().get("content-length") {
            length.to_str().unwrap().parse::<f64>().unwrap()
        } else {
            if let Ok(mut data) = state.lock() {
                *data = ButtonState::Failed;
            }
            return;
        };
        let p = String::from(env::current_exe().unwrap().parent().unwrap().to_str().unwrap());
        let p = format!("{}/maps/{}",p,map.file_name);
        let mut f = fs::File::create(p).unwrap();
        let mut dled = 0.0;

        loop {
            if let Ok(data) = state.lock() {
                if *data == ButtonState::Idle {
                    break;
                }
            }
            if dled >= total_size {
                if let Ok(mut data) = state.lock() {
                    *data = ButtonState::Finished;
                }
            }
            
            let mut buff = [0;4096];
            if let Ok(bytes) = response.read(&mut buff) {
                let _ = f.write(&mut buff);
                dled += bytes as f64;
            } else {
                continue;
            }
        }
    }
    pub fn get_map_info(&self) -> String {
        format!("{}\ncreated by {}",self.map_name,self.author)
    }

    pub fn get_icon_path(&self) -> String {
        let p = String::from(env::current_exe().unwrap().parent().unwrap().to_str().unwrap());
        format!("{}/icons/{}.png",p, self.max_players[0..2].trim() )
    }
}

pub fn crawl(c: Arc<reqwest::Client>,wc3db: Arc<Mutex<Vec<WC3Map>>>,i: usize) {

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
            let max_players = class.find(Name("img")).nth(0).unwrap().attr("alt").unwrap().to_owned().split_whitespace().next().unwrap().to_owned();           
            let map_link = class.find(Name("a")).nth(1).unwrap().attr("href").unwrap().to_owned();
            let file_name = class.find(Name("a")).nth(1).unwrap().text();
            let mut m =  WC3Map::new(author, map_name, max_players,map_link,file_name);
            wc3db.lock().unwrap().push(m);
        }
}