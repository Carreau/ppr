use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct A {
    value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct B {
    value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "c")]
enum AorB {
    X(A),
    Y(B),
}

#[derive(Debug, Serialize, Deserialize)]
struct Section {
    children: Option<Vec<String>>,
    tag: AorB,
}

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    #[serde(rename = "_content")]
    content: Value,
    refs: Vec<Value>,
    ordered_sections: Vec<Value>,
    see_also: Vec<Value>,
    aliases: Vec<Value>,
    item_file: String,
    item_line: i32,
    item_type: String,
    version: String,
    logo: String,
    example_section_data: Value,
}

fn read_data_from_file<P: AsRef<Path>>(path: P) -> Document {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

fn main() {
    //let sec = Section {
    //    children: Some(vec!["a".to_owned(), "b".to_owned()]),
    //    tag: AorB::X(A { value: 1 }),
    //};
    //println!("Hello, world! {:?}", &sec);

    //let data = serde_json::to_string(&sec).unwrap();
    //println!("Hello, world! {}", data);

    for mp in glob("/Users/bussonniermatthias/.papyri/ingest/*/*/module/*.json").unwrap()
    //.take(50)
    {
        if let Ok(p) = mp {
            //println!("{:?}", read_data_from_file(p));
            print!(".")
        }
    }
}
