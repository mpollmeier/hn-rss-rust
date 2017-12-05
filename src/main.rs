extern crate hyper;
extern crate xml;
extern crate select;

use hyper::Client;
use hyper::client::Response;

use std::fs::File;
use std::io::BufReader;
use std::env;
use xml::reader;
use xml::writer::{EmitterConfig, XmlEvent};

use select::document::Document;
use select::predicate::{Predicate, Class, Name};

#[derive(Debug)]
struct Article { 
    title: String,
    link: String
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let outfile: String = 
        if args.len() > 1 {
            args[1].to_string()
        } else {
            String::from("hn-scraper-scraper.xml")
        };
    println!("writing results to {}", outfile);

    let file = File::open("hn-index2.rss").unwrap();
    let file = BufReader::new(file);
    let parser = reader::EventReader::new(file);

    // let res = Client::new().get("http://www.daemonology.net/hn-daily/index.rss").send().unwrap();
    // let parser = reader::EventReader::new(res);

    let articles: Vec<Article> = extract_description_cdatas(parser)
        .into_iter()
        .map(|desc|Document::from(&desc[..]))
        .flat_map(parse_description_document)
        .collect();
    
    let mut outfile = File::create(outfile).expect("Unable to create file");
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut outfile);

    let error_msg = String::from("error while writing element");
    writer.write(XmlEvent::start_element("rss").attr("version", "2.0")).expect(&error_msg);
    writer.write(XmlEvent::start_element("channel")).expect(&error_msg);

    writer.write(XmlEvent::start_element("title")).expect(&error_msg);
    writer.write(XmlEvent::characters("Hacker News Scraper Scraper")).expect(&error_msg);
    writer.write(XmlEvent::end_element()).expect(&error_msg);

    for article in &articles {
        writer.write(XmlEvent::start_element("item")).expect(&error_msg);
        writer.write(XmlEvent::start_element("title")).expect(&error_msg);
        writer.write(XmlEvent::characters(&article.title)).expect(&error_msg);
        writer.write(XmlEvent::end_element()).expect(&error_msg);
        writer.write(XmlEvent::start_element("link")).expect(&error_msg);
        writer.write(XmlEvent::characters(&article.link)).expect(&error_msg);
        writer.write(XmlEvent::end_element()).expect(&error_msg);
        writer.write(XmlEvent::start_element("guid")).expect(&error_msg);
        writer.write(XmlEvent::characters(&article.link)).expect(&error_msg);
        writer.write(XmlEvent::end_element()).expect(&error_msg);
        writer.write(XmlEvent::end_element()).expect(&error_msg);
    }
    writer.write(XmlEvent::end_element()).expect(&error_msg);
    writer.write(XmlEvent::end_element()).expect(&error_msg);
}

fn parse_description_document(document: Document) -> Vec<Article> {
    document.find(Class("storylink").descendant(Name("a")))
        .into_iter()
        .map(|node|
          Article {
              title: node.text(),
              link: node.attr("href").unwrap().to_string()
          }
        ).collect()
}

fn extract_description_cdatas(parser: reader::EventReader<BufReader<File>>) -> Vec<String> {
// fn extract_description_cdatas(parser: reader::EventReader<Response>) -> Vec<String> {
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
