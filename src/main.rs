// Pull in the necessary crates
extern crate iron;
extern crate router;

// Define which parts of the standard library and the imported crates will be used
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::env;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;

// Populate some text files (helps with Heroku configuration to do it this way)
fn populate_files() {
    let mut f0 = File::create("hello_world.txt").unwrap();
    f0.write_all(b"Hello world!");
    let mut f1 = File::create("foo.txt").unwrap();
    f1.write_all(b"I'm the foo.txt file!");
    let mut f2 = File::create("bar.txt").unwrap();
    f2.write_all(b"I'm the bar.txt file!");
    let mut f3 = File::create("numbers.txt").unwrap();
    f3.write_all(b"0123456789");
    let mut f4 = File::create("yay.txt").unwrap();
    f4.write_all(b"It works!");
}

// The empty request case: returns a flagged JSON string
fn noop(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, format!("{{ \"file\": \"none\", \"content\": \"none\" }}")));
    Ok(resp)
}

// Pull the selected file, extract its text, format into a JSON string, then send it back to the browser
fn get_json_from_file(req: &mut Request) -> IronResult<Response> {
    
    // Get the name of the file from the request URL
    let params = req.extensions.get::<Router>().unwrap();
    let filename = params.find("name").unwrap();
    
    // Retrieve the file and its text content
    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    // Send it to the browser as a JSON-formatted string
    let resp = Response::with((status::Ok, format!("{{ \"file\": \"{}\", \"content\": \"{}\" }}", filename, content)));
    Ok(resp)
}

// For Heroku configuration
fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}

// Set up the app's router
fn main() {
    populate_files();
    let mut router = Router::new();
    router.get("/", noop);
    router.get("/:name", get_json_from_file);
    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();
}