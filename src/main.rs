use glob::glob;
#[allow(unused_imports)]
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
#[allow(unused_imports)]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use std::collections::HashMap;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use askama::Template;

use ::ppr::Point;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    #[serde(rename = "_content")]
    content: HashMap<String, Option<MaybeTL>>,
    refs: Vec<String>,
    ordered_sections: Vec<String>,
    see_also: Vec<SeeAlsoItem>,
    aliases: Vec<String>,
    item_file: Option<String>,
    item_line: Option<i32>,
    item_type: Option<String>,
    version: Option<String>,
    logo: Option<String>,
    example_section_data: Option<ExampleSectionData>,
    signature: Option<String>,
    references: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Section {
    children: Vec<TopLevelBlock>,
}

#[serde(untagged)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum MaybeTL {
    Section(Section),
    S(String),
    L(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExampleSectionData {
    children: Option<Vec<TopLevelBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Param {
    param: String,
    type_: String,
    desc: Vec<TopLevelBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(deny_unknown_fields)]
enum TopLevelBlock {
    Paragraph(Paragraph),
    DefList(DefList),
    Code(Code),
    BlockDirective(BlockDirective),
    Fig(Fig),
    Words(Words),
    Directive(Directive),
    Verbatim(Verbatim),
    Math(Math),
    Param(Param),
    BlockVerbatim(BlockVerbatim),
    Example(Example),
    Link(Link),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Link {
    value: String,
    reference: RefInfo,
    kind: String,
    exists: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RefInfo {
    module: Option<String>,
    version: Option<String>,
    kind: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Paragraph {
    children: Vec<TopLevelBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SeeAlsoItem {
    name: Ref,
    descriptions: Vec<Paragraph>,
    #[serde(rename = "type")]
    ty: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Ref {
    name: String,
    #[serde(rename = "ref")]
    target: Option<String>,
    exists: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Math {
    value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DefList {
    children: Vec<DefListItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Line {
    #[serde(rename = "_line")]
    line: String,
    #[serde(rename = "_number")]
    number: u64,
    #[serde(rename = "_offset")]
    offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Lines {
    #[serde(rename = "_lines")]
    lines: Vec<Line>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DefListItem {
    lines: Lines,
    wh: Lines,
    ind: Lines,
    dt: Paragraph, // todo wrong
    dd: Paragraph,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(deny_unknown_fields)]
struct CodeEntry {
    token: String,
    target: Option<String>,
    pygc: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Code {
    ce_status: String,
    entries: Vec<CodeEntry>, // List[Tuple[Optional[str]]]
    out: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Example {
    lines: Lines,
    wh: Lines,
    ind: Lines,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BlockDirective {
    lines: Lines,
    wh: Lines,
    ind: Lines,

    directive_name: String,
    args0: Vec<String>,
    inner: Option<Paragraph>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BlockVerbatim {
    lines: Lines,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Directive {
    value: Vec<String>,
    domain: Option<String>,
    role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Words {
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Verbatim {
    value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Fig {
    value: String,
}

fn read_data_from_file<P: AsRef<Path>>(path: P) -> Result<Document, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

#[derive(Template)] // this will generate the code...
#[template(path = "base.html")] // using the template in this path, relative
                                // to the `templates` dir in the crate root
struct HTMLTemplate<'a> {
    name: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let pth: Vec<_> = glob("/Users/bussonniermatthias/.papyri/ingest/*/*/module/*.json")?.collect();

    let bar = ProgressBar::new(pth.len() as u64);

    pth.iter().progress_with(bar).for_each(|mp| {
        if let Ok(p) = mp {
            //println!("{:?}", p.display());
            //let val = format!("{:?}", p.display());
            //println!("{}", HTMLTemplate { name: val.as_str() }.render().unwrap());
            let document = read_data_from_file(p).unwrap();
            if let Some(example) = document.example_section_data {
                if let Some(ee) = example.children {
                    for c in ee {
                        match c {
                            TopLevelBlock::Code(code) => {
                                code.entries
                                    .into_iter()
                                    .for_each(|entry| match entry.target {
                                        Some(e) => {
                                            if e.is_empty() == false {
                                                // println!("  {}", &e);
                                                ()
                                            }
                                        }
                                        None => (),
                                    })
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    });
    Ok(())
}
