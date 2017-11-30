extern crate hyper;
extern crate xml;
extern crate select;

// use std::io::Read;
// use hyper::Client;
// use std::io;
// use hyper::client::Response;

use std::fs::File;
use std::io::BufReader;
use xml::reader;
// use xml::writer;

use select::document::Document;
use select::predicate::{Predicate, Class, Name};

fn main() {
    // let body = get().unwrap();
    // let body = read_file("/home/mp/hn-index.rss").unwrap();
    // println!("{}", body);

    let file = File::open("hn-index.short.rss").unwrap();
    let file = BufReader::new(file);
    let parser = reader::EventReader::new(file);

    // let mut outfile = File::create("output.xml").unwrap();
    // let mut writer = writer::EmitterConfig::new().perform_indent(true).create_writer(&mut outfile);

    // let descriptions: Vec<String> = extract_description_cdatas(parser);
    // let descriptions = descriptions.into_iter().map(|desc|Document::from(&desc[..]));
    let articles: Vec<Article> = extract_description_cdatas(parser)
        .into_iter()
        .map(|desc|Document::from(&desc[..]))
        .flat_map(parse_description_document)
        .collect();
    
    for x in &articles {
        println!("{:?}", x);
    }

}

fn parse_description_document(document: Document) -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();

    for node in document.find(Class("storylink").descendant(Name("a"))) {
        articles.push(
            Article {
                title: node.text(),
                link: node.attr("href").unwrap().to_string()
            }
        );
    }

    articles
}

// fn parse_description_cdata2(cdata: String) -> Vec<Article> {
//     // println!("X {}", cdata);
//     let parser = reader::EventReader::from_str(&cdata);

//     let mut articles: Vec<Article> = Vec::new();
//     let mut inside_storylink = false;
//     let mut current_story = String::new();
//     let mut current_href = String::new();
//     for e in parser {
//         match e {
//             Ok(reader::XmlEvent::StartElement { name, attributes, .. }) => {
//                 let href = attributes.iter().find(|attr| attr.name.local_name == "href");
//                 if href.is_some() {
//                     // println!("{}", href.unwrap());
//                 }
//                 if attributes.iter().find(|attr| attr.value == "storylink").is_some() {
//                     inside_storylink = true;
//                 }
//                 if inside_storylink && name.local_name == "a" {
//                     let href = attributes.iter().find(|attr| attr.name.local_name == "href");
//                     if href.is_some() {
//                         current_href = href.unwrap().value.clone();
//                     }
//                 }
//             }
//             Ok(reader::XmlEvent::Characters(chars)) => {
//                 if inside_storylink {
//                     current_story = String::from(chars);
//                 }
//             }
//             Ok(reader::XmlEvent::EndElement { name }) => {
//                 if name.local_name == "span" && inside_storylink {
//                     articles.push(
//                         Article {
//                             title: current_story.clone(),
//                             link: current_href.clone()
//                         }
//                     );
//                     inside_storylink = false;
//                 }
//             }
//             Err(e) => {
//                 println!("err: {}", e);
//             }
//             _ => {}
//         }
//     }

//     articles
// }

fn extract_description_cdatas(parser: reader::EventReader<BufReader<File>>) -> Vec<String> {
    let mut contents: Vec<String> = Vec::new();
    for e in parser {
        match e {
            Ok(reader::XmlEvent::CData(chars)) => {
                contents.push(chars);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    contents
}

#[derive(Debug)]
struct Article { 
    title: String,
    link: String
}

// fn indent(size: usize) -> String {
//     const INDENT: &'static str = "    ";
//     (0..size).map(|_| INDENT)
//              .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
// }

// fn get() -> Result<String, hyper::Error> {
//     let client = Client::new();
//     let mut res = client.get("http://www.daemonology.net/hn-daily/index.rss").send().unwrap();
//     assert_eq!(res.status, hyper::Ok);

//     let mut body = String::new();
//     res.read_to_string(&mut body)
//         .expect("failed to parse body");
//     Ok(body)
// }


// TODO remove
// use std::fs::File;
// fn read_file(path: &str) -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open(path)?.read_to_string(&mut s);  // `s` contains the contents of "foo.txt"
//     Ok(s)
// }
