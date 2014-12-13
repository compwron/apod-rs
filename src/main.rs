#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate curl;

use curl::http;
use std::str;
use std::string::String;
use std::io::{Command, File};
use std::os;
use std::cell::RefCell;

static APOD_BASE_URL: &'static str = "http://apod.nasa.gov/apod/";

struct MemoryPage
{
    code: uint,
    headers: http::Headers,
    body: Vec<u8> 
}

struct Apod<'a> {
    handle: RefCell<http::Handle>
}


impl<'a> Apod<'a> {

    fn new() -> Apod<'a> {
        Apod { handle: RefCell::new(http::handle().verbose()) }
    }

    fn get_page(&self, url: &str) -> MemoryPage {
        let resp = self.handle.borrow_mut().get(url)
                    .header("User-Agent", "apod-rs/0.1 github.com/supr/apod-rs")
                    .exec().unwrap();
        MemoryPage { code: resp.get_code(), headers: resp.get_headers().clone(), body: resp.move_body() }
    }

    fn get_image_url<'a>(&self, page: &MemoryPage) -> Option<&'a str> {
        let rex = regex!("<a href=\"(image.*)\"");
        let body = str::from_utf8(page.body.as_slice()).unwrap();
        match rex.is_match(body) {
            true => {
                Some(rex.captures(body).unwrap().at(1))
            },
            false => None
        }
    }

    fn download_page(&self, to: String, url: String) -> String {
        let page = self.get_page(url.as_slice());
        let web_path = Path::new(url);
        let file_name = web_path.filename().unwrap();

        let mut file = File::create(&Path::new(format!("{}/{}", to, str::from_utf8(file_name).unwrap())));
        file.write(page.body.as_slice_());

        format!("{}/{}", to, str::from_utf8(file_name).unwrap())
    }

    fn set_wallpaper(&self, file: String) {
        let file_path = format!("file://{}", file);
        let args = vec!["set", "org.gnome.desktop.background", "picture-uri", file_path.as_slice()];
        Command::new("gsettings").args(args.as_slice_()).spawn();
    }
}

fn main() {
    let apod = Apod::new();
    let page = apod.get_page(APOD_BASE_URL);

    if let Some(url) = apod.get_image_url(&page) {
        let downloaded_file = apod.download_page(format!("{}/Pictures", os::homedir().unwrap().display()), format!("{}{}", APOD_BASE_URL, url));
        apod.set_wallpaper(downloaded_file);
    }
}
