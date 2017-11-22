extern crate hyper;
extern crate xml;

// use std::io::Read;
// use hyper::Client;
// use std::io;
// use hyper::client::Response;

use std::fs::File;
use std::io::BufReader;
use xml::reader;
// use xml::writer;

fn main() {
    // let body = get().unwrap();
    // let body = read_file("/home/mp/hn-index.rss").unwrap();
    // println!("{}", body);

    let file = File::open("hn-index.short.rss").unwrap();
    let file = BufReader::new(file);
    let parser = reader::EventReader::new(file);

    // let mut outfile = File::create("output.xml").unwrap();
    // let mut writer = writer::EmitterConfig::new().perform_indent(true).create_writer(&mut outfile);

    let descriptions = extract_description_cdatas(parser);
    // println!("desc.len: {}", descriptions.len());
    let articles: Vec<Article> = descriptions.into_iter().flat_map(parse_description_cdata).collect();
    // println!("art.len: {}", articles.len());
}

fn parse_description_cdata(cdata: String) -> Vec<Article> {
    let mut articles: Vec<Article> = Vec::new();

    let parser = reader::EventReader::from_str(&cdata);

    let mut inside_storylink = false;
    let mut inside_anchor = false;
    for e in parser {
        match e {
            Ok(reader::XmlEvent::StartElement { name, attributes, .. }) => {
                inside_anchor = name.local_name == "a";

                // TODO: this doesn't seem to work yet
                inside_storylink =
                    attributes.iter().find(|attr| attr.name.local_name == "storylink").is_some();

                // TODO: get href from attributes
            }
            Ok(reader::XmlEvent::Characters(chars)) => {
                if inside_storylink {
                  println!("<inside storylink>");
                } else {
                  println!("<outside storylink>");
                }
                if inside_anchor {
                  println!("<inside a>");
                } else {
                  println!("<outside a>");
                }
                println!("{}", chars);
                if inside_anchor {
                  println!("</inside a>");
                } else {
                  println!("</outside a>");
                }
                if inside_storylink {
                  println!("</inside storylink>");
                } else {
                  println!("</outside storylink>");
                }
                // TODO: get title from anchor body
            }
            Ok(reader::XmlEvent::EndElement { name }) => {
                if name.local_name == "a" {
                    inside_anchor = false;
                }
                if name.local_name == "span" {
                    inside_storylink = false;
                }
            }
            _ => {}
        }
    }

    // articles.push(
    //     Article {
    //         title: String::from("test-title"),
    //         link: String::from("test-link")
    //     }
    // );
    articles
}

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

// TODO: construct Vec<Article> from description cdata
struct Article { 
    // title: String,
    // link: String
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
