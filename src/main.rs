use glob::glob;
#[allow(unused_imports)]
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
#[allow(unused_imports)]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
//use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use askama::Template;

use ::ppr::*;

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

fn cc(b: &TopLevelBlock, count: u64) -> u64 {
    return match b {
        TopLevelBlock::Paragraph(p) => {
            p.inline.iter().map(|item| cc(item, count)).sum::<u64>()
                //+ (p.inner.iter().map(|item| cc(item, count)).sum())
        }
        TopLevelBlock::BulletList(b) => b.value.iter().map(|item| cc(item, count)).sum(),
        TopLevelBlock::EnumeratedList(b) => b.value.iter().map(|item| cc(item, count)).sum(),
        TopLevelBlock::BlockQuote(_) => 0,
        TopLevelBlock::DefList(_) => 1,
        TopLevelBlock::Code(_) => 0,
        TopLevelBlock::BlockDirective(_) => 0,
        TopLevelBlock::Fig(_) => 0,
        TopLevelBlock::Words(_) => 0,
        TopLevelBlock::Directive(_) => 0,
        TopLevelBlock::Verbatim(_) => 0,
        TopLevelBlock::Math(_) => 0,
        TopLevelBlock::Param(_) => 0,
        TopLevelBlock::BlockVerbatim(_) => 0,
        TopLevelBlock::Example(_) => 0,
        TopLevelBlock::Link(_) => 0,
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let pth: Vec<_> = glob("/Users/bussonniermatthias/.papyri/ingest/*/*/module/*.json")?.collect();

    let bar = ProgressBar::new(pth.len() as u64);
    let mut count: u64 = 0;
    pth.iter().progress_with(bar).for_each(|mp| {
        if let Ok(p) = mp {
            //println!("{:?}", p.display());
            //let val = format!("{:?}", p.display());
            //println!("{}", HTMLTemplate { name: val.as_str() }.render().unwrap());
            let document = read_data_from_file(p).unwrap();
            for (_, v) in document.content {
                for sec in v.unwrap().children {
                    let var_name = cc(&sec, count);
                    count += var_name;
                }
            }
            // if let Some(example) = document.example_section_data {
            //     if let Some(ee) = example.children {
            //         for c in ee {
            //             match c {
            //                 TopLevelBlock::Code(code) => {
            //                     code.entries
            //                         .into_iter()
            //                         .for_each(|entry| match entry.target {
            //                             Some(e) => {
            //                                 if e.is_empty() == false {
            //                                     // println!("  {}", &e);
            //                                     ()
            //                                 }
            //                             }
            //                             None => (),
            //                         })
            //                 }
            //                 _ => (),
            //             }
            //         }
            //     }
            // }
        }
    });
    println!("{:?}", count);
    Ok(())
}
