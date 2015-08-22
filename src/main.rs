extern crate iron;
extern crate router;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::env;
use std::collections::BTreeMap;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;

fn populate_files() {
    let mut f0 = File::create("hello_world.txt").unwrap();
    f0.write_all(b"Hello world!");
    let mut f1 = File::create("foo.txt").unwrap();
    f1.write_all(b"I'm the foo.txt file!");
    let mut f2 = File::create("bar.txt").unwrap();
    f2.write_all(b"I'm the bar.txt file!");
}

fn noop(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, format!("{{ \"file\": \"none\", \"content\": \"none\" }}")));
    Ok(resp)
}

fn get_json_from_file(req: &mut Request) -> IronResult<Response> {
    
    let params = req.extensions.get::<Router>().unwrap();
    let filename = params.find("name").unwrap();
    
    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let resp = Response::with((status::Ok, format!("{{ \"file\": \"{}\", \"content\": \"{}\" }}", filename, content)));
    Ok(resp)
}

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}

fn main() {
    populate_files();
    let mut router = Router::new();
    router.get("/", noop);
    router.get("/:name", get_json_from_file);
    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();
}