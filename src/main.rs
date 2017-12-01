extern crate hyper;
extern crate xml;
extern crate select;

// use std::io::Read;
// use hyper::Client;
// use std::io;
// use hyper::client::Response;

use std::fs::File;
use std::io::{BufReader, Write, BufWriter};
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

    let articles: Vec<String> = extract_description_cdatas(parser)
        .into_iter()
        .map(|desc|Document::from(&desc[..]))
        .flat_map(parse_description_document)
        .map(to_rss_item)
        .collect();
    
    let outfile = File::create("hn-scraper-scraper.xml").expect("Unable to create file");
    let mut outfile = BufWriter::new(outfile);

    let rss_start =
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <rss version=\"2.0\">
           <channel>
             <title>Hacker News Scraper Scraper</title>
             <link>http://www.daemonology.net/hn-daily/</link>\n";
    outfile.write_all(rss_start.as_bytes()).expect("Unable to write data");

    for article in &articles {
        outfile.write_all(article.as_bytes()).expect("Unable to write data");
    }

    let rss_end = "</channel></rss>";
    outfile.write_all(rss_end.as_bytes()).expect("Unable to write data");
}

// TODO: create document, not string
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

// fn get() -> Result<String, hyper::Error> {
//     let client = Client::new();
//     let mut res = client.get("http://www.daemonology.net/hn-daily/index.rss").send().unwrap();
//     assert_eq!(res.status, hyper::Ok);

//     let mut body = String::new();
//     res.read_to_string(&mut body)
//         .expect("failed to parse body");
//     Ok(body)
// }
