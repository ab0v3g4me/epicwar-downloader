extern crate reqwest;
extern crate gtk;
extern crate gdk;

use std::fs;
use std::env;
use std::io::{Write,Read};
use std::clone;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};

pub struct WC3Map {
    author: String,
    map_name: String,
    max_players: String,
    pub map_link: String,
    pub file_name: String,
    pub dl_started: Arc<Mutex<bool>>,
}

impl WC3Map {
    const BASEURL: &'static str = "https://www.epicwar.com";
    pub fn new(author: String,map_name: String, max_players: String,map_link: String,file_name: String,dl_started: Arc<Mutex<bool>>) -> Self {
        WC3Map {
            author: author,
            map_name: map_name,
            max_players: max_players,
            map_link: map_link,
            file_name: file_name,
            dl_started: dl_started,
        }
    }
    pub fn download(file_name: String,link: String,tx: mpsc::SyncSender<f64>) -> Result<String,String> {
        let fullurl = format!("{}{}",Self::BASEURL, link);
/*        let c = reqwest::Client::new();
        let mut r = match c.get(&fullurl[..])
        .header(reqwest::header::UserAgent::new("Mozilla/5.0 (X11; Linux x86_64; rv:58.0) Gecko/20100101 Firefox/58.0"))
        .send(){
            Ok(r) => r,
            Err(_) => return Err("Failed".to_owned()),
        };*/

        let mut r = match reqwest::get(fullurl.as_str()) {
                Ok(r) => r,
                Err(_) => return Err("Download failed.".to_owned()),
        };

        let mut total_size = 1.0;
        match r.headers().get::<reqwest::header::ContentLength>() {
              Some(length) => { total_size = length.to_le() as f64; },
              None => (),
        };
        let p = String::from(env::current_exe().unwrap().parent().unwrap().to_str().unwrap());
        let p = format!("{}/maps/{}",p,file_name.clone());
        let mut f = fs::File::create(p).unwrap();
        let mut dled = 0.0;

        loop {
            let mut buff = [0;4096];

            match r.read(&mut buff) {
                Ok(0) => break,
                Ok(b) => { let _ = f.write(&mut buff);dled += b as f64;},
                Err(_) => break,
            };
            tx.send( dled / total_size ).unwrap();
        }

        Ok("Done".to_owned())
    }
    pub fn get_map_info(&self) -> String {
        format!("{} by {}",self.map_name,self.author)
    }

    pub fn get_icon_path(&self) -> String {
        let p = String::from(env::current_exe().unwrap().parent().unwrap().to_str().unwrap());
        let ret_path = format!("{}/icons/{}.gif",p, self.max_players[0..2].trim() );
        ret_path
    }
}

impl clone::Clone for WC3Map {
    fn clone(&self) -> WC3Map {
        WC3Map::new(self.author.clone(),
                    self.map_name.clone(),
                    self.max_players.clone(),
                    self.map_link.clone(),
                    self.file_name.clone(),
                    self.dl_started.clone())
    }
}
