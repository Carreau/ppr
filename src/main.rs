use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::collections::HashMap;

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
    content: HashMap<String, Value>,
    refs: Vec<Value>,
    ordered_sections: Vec<Value>,
    see_also: Vec<Value>,
    aliases: Vec<Value>,
    item_file: Option<String>,
    item_line: Option<i32>,
    item_type: Option<String>,
    version: Option<String>,
    logo: Option<String>,
    example_section_data: Option<ExampleSectionData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExampleSectionData {
    children: Option<Vec<TLB>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum TLB {
    Paragraph(Paragraph),
    DefList(DefList),
    Code(Code),
}

#[derive(Debug, Serialize, Deserialize)]
struct Paragraph {
    children: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DefList {
    children: Vec<DefListItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DefListItem {
    lines: Value,
    wh: Value,
    ind: Value,
    dt: Paragraph, // todo wrong
    dd: Paragraph,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
struct CodeEntry {
    token: String,
    target: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Code {
    ce_status: String,
    entries: Vec<CodeEntry>, // List[Tuple[Optional[str]]]
    out: String,
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

    for mp in glob("/Users/bussonniermatthias/.papyri/ingest/*/*/module/*.json")
        .unwrap()
        .take(500)
    {
        if let Ok(p) = mp {
            //println!("{:?}", p.display());
            let document = read_data_from_file(p);
            if let Some(example) = document.example_section_data {
                if let Some(ee) = example.children {
                    for c in ee {
                        match c {
                            TLB::Paragraph(_) => println!("{:?}", "Paragraph"),
                            TLB::DefList(_) => println!("{:?}", "Deflist"),
                            TLB::Code(code) => {
                                println!("{:?}", code.ce_status);
                                println!("    {:?}", code.entries);
                                println!("    {:?}", code.out);
                            }
                        }
                    }
                }
            }
        }
    }
}
