#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate serde_json;


use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use std::env::args;

use serde::de::{Deserialize, DeserializeOwned};
use serde_yaml::from_str as yaml_from_str;

mod examples;
mod utils;

use examples::json_example::run as run_json_example;
use examples::yaml_example::run as run_yaml_example;

fn main() {
    let flow = args().nth(1).expect("Please enter the requested flow");
    match flow.as_str() {
        "json" => run_json_example(args().nth(2).expect("Please enter a path for the json file")),
        "yaml" => run_yaml_example(args().nth(2).expect("Please enter a path for the yaml file")),
        other => panic!(format!("`{}` is not a recognized flow. Please enter either `json` or `yaml`", other))
    }
}
