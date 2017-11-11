extern crate hyper;

use std::io::Read;
use hyper::Client;
use std::io;
// use hyper::client::Response;

fn main() {
    // let body = get().unwrap();
    let body = read_file("/home/mp/hn-index.rss").unwrap();
    // println!("{}", body);
}

// fn get() -> Result<String, hyper::Error> {
//     let client = Client::new();
//     let mut res = client.get("http://example.org").send().unwrap();
//     assert_eq!(res.status, hyper::Ok);

//     let mut body = String::new();
//     res.read_to_string(&mut body)
//         .expect("failed to parse body");
//     Ok(body)
// }


// TODO remove
use std::fs::File;
fn read_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s);  // `s` contains the contents of "foo.txt"
    Ok(s)
}
