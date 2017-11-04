use serde;
use serde_json;
use serde_derive;

use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use std::env::args;
use std::io::Error;
use utils::io::read_content_of_file_to_string;

use serde::de::{Deserialize, DeserializeOwned};
use serde_yaml::from_str as yaml_from_str;


pub fn run(filepath: String) {
    let mut string = String::new();
    read_content_of_file_to_string(filepath.as_str(), &mut string);
    let thingy = serde_json::from_str::<Thingy>(string.as_str()).expect("Couldn't parse the file");

    println!("\n\nhead = {}", thingy.head);

    thingy.requests.iter().for_each(|request| {
        println!("\tname = {}", request.name);
        if let Some(age) = request.age {
            println!("\t{} has the age property. age = {}", request.name, age);
        }
        println!();
    });
}

#[derive(Deserialize, Serialize)]
pub struct Thingy {
    head: String,
    requests: Vec<PrivateRequest>
}

#[derive(Deserialize, Serialize)]
pub struct PrivateRequest {
    name: String,
    age: Option<u32>,
    active: bool
}
