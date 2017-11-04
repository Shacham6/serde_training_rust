use serde;
use serde_derive;
use serde_yaml;

use std::env::args;
use std::vec::Vec;

use utils::read_content_of_file;

pub fn run(filepath: String) {
    let file_content = read_content_of_file(filepath.as_str()).expect("Failed to parse the json file");
    let content = serde_yaml::from_str::<Vec<Person>>(file_content.as_str()).expect("The content of the file is invalid");

    println!("MANAGED TO PARSE THE YAML FILE!!\n");
    content.iter().for_each(|person| {
        println!("-");
        println!("\tfullname: {}", person.fullname);
        println!("\tage: {}", person.age);
    })
}

#[derive(Deserialize, Serialize)]
pub struct Person {
    fullname: String,
    age: usize
}
