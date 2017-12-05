extern crate hyper;
extern crate xml;
extern crate select;

use hyper::Client;
use hyper::client::Response;

use std::fs::File;
use std::io::{BufReader, Write, BufWriter};
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
    let file = File::open("hn-index2.rss").unwrap();
    let file = BufReader::new(file);
    let parser = reader::EventReader::new(file);

    // let res = Client::new().get("http://www.daemonology.net/hn-daily/index.rss").send().unwrap();
    // let parser = reader::EventReader::new(res);

    let articles: Vec<Article> = extract_description_cdatas(parser)
        .into_iter()
        .map(|desc|Document::from(&desc[..]))
        .flat_map(parse_description_document)
        // .map(to_rss_item)
        .collect();
    
    let mut outfile = File::create("hn-scraper-scraper.xml").expect("Unable to create file");
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

    // start string writer
    // let mut outfile = BufWriter::new(outfile);

    // let rss_start =
    //     "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
    //      <rss version=\"2.0\">
    //        <channel>
    //          <title>Hacker News Scraper Scraper</title>\n";
    // outfile.write_all(rss_start.as_bytes()).expect("Unable to write data");

    // for article in &articles {
    //     outfile.write_all(article.as_bytes()).expect("Unable to write data");
    // }

    // let rss_end = "</channel></rss>";
    // outfile.write_all(rss_end.as_bytes()).expect("Unable to write data");
}

fn to_rss_item(article: Article) -> String {
    format!("<item><title>{}</title><link>{}</link><guid>{}</guid></item>\n", article.title, article.link, article.link)
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
