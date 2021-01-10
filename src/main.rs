use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    children: Option<Vec<TopLevelBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum TopLevelBlock {
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

fn read_data_from_file<P: AsRef<Path>>(path: P) -> Result<Document, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

fn main() -> Result<(), Box<dyn Error>> {
    //let sec = Section {
    //    children: Some(vec!["a".to_owned(), "b".to_owned()]),
    //    tag: AorB::X(A { value: 1 }),
    //};
    //println!("Hello, world! {:?}", &sec);

    //let data = serde_json::to_string(&sec).unwrap();
    //println!("Hello, world! {}", data);

    for mp in glob("/Users/bussonniermatthias/.papyri/ingest/*/*/module/*.json").unwrap()
    //.take(500)
    {
        if let Ok(p) = mp {
            //println!("{:?}", p.display());
            let document = read_data_from_file(p)?;
            if let Some(example) = document.example_section_data {
                if let Some(ee) = example.children {
                    for c in ee {
                        match c {
                            TopLevelBlock::Paragraph(_) => (),
                            TopLevelBlock::DefList(_) => (),
                            TopLevelBlock::Code(code) => {
                                code.entries
                                    .into_iter()
                                    .for_each(|entry| match entry.target {
                                        Some(e) => {
                                            if e.is_empty() == false {
                                                println!("{}", e)
                                            }
                                        }
                                        None => (),
                                    })
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
