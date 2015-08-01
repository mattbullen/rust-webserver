extern crate iron;
extern crate router;
extern crate rustc_serialize;

use std::str::FromStr;
use std::env;
use iron::{Iron, Request, Response, IronResult};
use router::Router;
use iron::status;
use rustc_serialize::json::{self, Json};

#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

#[derive(RustcDecodable, RustcEncodable)]
struct foo {
    test: String,
}
    
// Serves a string to the user.  Try accessing "/".
fn hello(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, "Default string!"));
    Ok(resp)
}

// Serves a customized string to the user.  Try accessing "/world".
fn hello_name(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let zzz = params.find("name").unwrap();
    
    
    //let data = Json::from_str(zzz).unwrap();
    //let data_object = data.as_object().unwrap();
    //let name1 = data_object.get("test").unwrap();
    //let name2: String = json::decode(name1).unwrap();

    // Deserialize using `json::decode`
    //let decoded: Posted = json::decode(&zzz).unwrap();    
    //let s = decoded.test;
    
    //let ss = Json::find(&data, "test").unwrap();
    //let sss = Json::as_string(&ss).unwrap();
    //let s1 = String::from_str(sss);    
    
    let json = zzz.to_string();
    let decoded: foo = json::decode(&json).unwrap();
    let test = decoded.get("test").unwrap();
    
    let object = TestStruct {
        data_int: 1,
        data_str: test.to_string(),
        data_vector: vec![2,3,4,5],
    };

    // Serialize using `json::encode
    //let obj = Json::from_str(&object.to_json()).unwrap();
    let encoded = json::encode(&object).unwrap();

    // Deserialize using `json::decode`
    // let decoded: TestStruct = json::decode(&encoded).unwrap();    
    
    // let resp = Response::with((status::Ok, format!("Custom string: {}!", name)));
    
    let resp = Response::with((status::Ok, encoded));
    
    Ok(resp)
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}

/// Configure and run our server.
fn main() {
    // Set up our URL router.
    let mut router = Router::new();
    router.get("/", hello);
    router.get("/:name", hello_name);

    // Run the server.
    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();
}