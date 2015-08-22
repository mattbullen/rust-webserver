extern crate iron;
extern crate router;
//extern crate rustc_serialize;

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
//use rustc_serialize::json::{self, Json, ToJson};
/*
#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

impl ToJson for TestStruct {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        // All standard types implement `to_json()`, so use it
        d.insert("data_int".to_string(), self.data_int.to_json());
        d.insert("data_str".to_string(), self.data_str.to_json());
        d.insert("data_vector".to_string(), self.data_vector.to_json());
        Json::Object(d)
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct Foo {
    test: String,
}
*/
// Serves a string to the user.  Try accessing "/".
fn hello(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, format!("{{ \"data_str\": \" \" }}")));
    /*
    let object = TestStruct {
        data_int: 1,
        data_str: "test".to_string(),
        data_vector: vec![2,3,4,5],
    };

    // Serialize using json::encode
    
    let json_obj: Json = object.to_json();
    let json_str: String = json_obj.to_string();
    let resp = Response::with((status::Ok, json_str));
    */
    Ok(resp)
}

// Serves a customized string to the user.  Try accessing "/world".
fn hello_name(req: &mut Request) -> IronResult<Response> {
    
    let params = req.extensions.get::<Router>().unwrap();
    let filename = params.find("name").unwrap();
    
    
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
    
    //let json = zzz.to_string();
    //let decoded: Foo = json::decode(&json).unwrap();
    //let test = decoded.test;
    
    //let object = TestStruct {
    //    data_int: 1,
    //    data_str: filename.to_string(),
    //    data_vector: vec![2,3,4,5],
    //}; 

    // Serialize using json::encode 
    
    //let json_obj: Json = object.to_json();
    //let json_str: String = json_obj.to_string();
    //let resp = Response::with((status::Ok, json_obj));
    
    //let encoded = json::encode(&object).unwrap();
    //let resp = Response::with((status::Ok, object));
    /*
    let path = Path::new("../in.txt");
    
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
        //Err(why) => Response::with((status::Ok, format!("{{ \"data_str\": \"{}\" }}", Error::description(&why)))),
        //Ok() => Response::with((status::Ok, format!("{{ \"data_str\": \"{}\" }}", s))),
    }
    */
    //let path = Path::new("../in.txt");
    //let mut file = File::open(&path).unwrap();
    //let content = file.read_to_string().unwrap();

let mut f = match File::create("foo.txt") {
    Err(e) => {
        println!("Couldn't open foo.txt");
    },
    Ok(f) => f,
};
f.write_all(b"Hello world!");    
    
    
    let mut file = File::open("foo.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    //let mut contents: Vec<u8> = Vec::new();
    //let result = file.read_to_end(&mut contents).unwrap();
    //let filestr = String::from_utf8(contents).unwrap();
    //print!("Contains: {}", filestr);
     
    let resp = Response::with((status::Ok, format!("{{ \"content\": \"{}\" }}", content)));
    Ok(resp)
}

fn get_server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(8080)
}

fn main() {
    let mut router = Router::new();
    router.get("/", hello);
    router.get("/:name", hello_name);
    Iron::new(router).http(("0.0.0.0", get_server_port())).unwrap();
}