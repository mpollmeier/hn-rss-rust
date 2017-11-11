extern crate hyper;

use std::io::Read;
use hyper::Client;
// use std::io;
// use hyper::client::Response;

fn main() {
    let body = get().unwrap();
    println!("{}", body);
}

fn get() -> Result<String, hyper::Error> {
    let client = Client::new();
    let mut res = client.get("http://example.org").send().unwrap();
    assert_eq!(res.status, hyper::Ok);

    let mut body = String::new();
    res.read_to_string(&mut body)
        .expect("failed to parse body");
    Ok(body)
}
