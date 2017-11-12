extern crate hyper;
extern crate xml;

use std::io::Read;
use hyper::Client;
// use std::io;
// use hyper::client::Response;

use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    // let body = get().unwrap();
    // let body = read_file("/home/mp/hn-index.rss").unwrap();
    // println!("{}", body);

    let file = File::open("/home/mp/hn-index.short.rss").unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
            }
            Ok(XmlEvent::Characters(chars)) => {
                println!("{}{}", indent(depth), chars);
            }
            Ok(XmlEvent::CData(chars)) => {
                println!("{}{}", indent(depth), chars);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

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
