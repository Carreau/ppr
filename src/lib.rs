//use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;
use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::collections::HashMap;

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    #[serde(rename = "_content")]
    pub content: HashMap<String, Option<MaybeTL>>,
    pub refs: Vec<Link>,
    pub ordered_sections: Vec<String>,
    pub see_also: Vec<SeeAlsoItem>,
    pub aliases: Vec<String>,
    pub item_file: Option<String>,
    pub item_line: Option<i32>,
    pub item_type: Option<String>,
    pub version: Option<String>,
    pub logo: Option<String>,
    pub example_section_data: Option<ExampleSectionData>,
    pub signature: Option<String>,
    pub references: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub children: Vec<TopLevelBlock>,
}

#[serde(untagged)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum MaybeTL {
    Section(Section),
    S(String),
    L(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExampleSectionData {
    pub children: Option<Vec<TopLevelBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    param: String,
    type_: String,
    desc: Vec<TopLevelBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(deny_unknown_fields)]
pub enum TopLevelBlock {
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
pub struct Paragraph {
    children: Vec<TopLevelBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeeAlsoItem {
    name: Ref,
    descriptions: Vec<Paragraph>,
    #[serde(rename = "type")]
    ty: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ref {
    name: String,
    #[serde(rename = "ref")]
    target: Option<String>,
    exists: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Math {
    value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefList {
    children: Vec<DefListItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Line {
    #[serde(rename = "_line")]
    line: String,
    #[serde(rename = "_number")]
    number: u64,
    #[serde(rename = "_offset")]
    offset: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lines {
    #[serde(rename = "_lines")]
    lines: Vec<Line>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DefListItem {
    lines: Lines,
    wh: Lines,
    ind: Lines,
    dt: Paragraph, // todo wrong
    dd: Paragraph,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(deny_unknown_fields)]
pub struct CodeEntry {
    pub token: String,
    pub target: Option<String>,
    pub pygc: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Code {
    pub ce_status: String,
    pub entries: Vec<CodeEntry>, // List[Tuple[Optional[str]]]
    pub out: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub lines: Lines,
    pub wh: Lines,
    pub ind: Lines,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlockDirective {
    pub lines: Lines,
    pub wh: Lines,
    pub ind: Lines,

    pub directive_name: String,
    pub args0: Vec<String>,
    pub inner: Option<Paragraph>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlockVerbatim {
    lines: Lines,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Directive {
    value: Vec<String>,
    domain: Option<String>,
    role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Words {
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verbatim {
    value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fig {
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    value: String,
    reference: RefInfo,
    kind: String,
    exists: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefInfo {
    module: Option<String>,
    version: Option<String>,
    kind: String,
    path: String,
}
// #[pyclass]
// #[derive(Clone)]
// pub struct Point {
//     #[pyo3(get, set)]
//     x: i64,
//     #[pyo3(get, set)]
//     y: i64,
// }

//#[pymethods]
//impl Point {
//    #[new]
//    fn new(x: i64, y: i64) -> Self {
//        Point { x, y }
//    }
//}
//
//#[pyclass]
//struct MyVec {
//    #[pyo3(get, set)]
//    start: Point,
//    #[pyo3(get, set)]
//    end: Point,
//}
//
//#[pymethods]
//impl MyVec {
//    #[new]
//    fn new(start: Point, end: Point) -> Self {
//        MyVec { start, end }
//    }
//}
//
///// A Python module implemented in Rust.
//#[pymodule]
//fn ppr(_py: Python, m: &PyModule) -> PyResult<()> {
//    //    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//    //    m.add_class::<Point>()?;
//    //    m.add_class::<MyVec>()?;
//    //
//    Ok(())
//}
