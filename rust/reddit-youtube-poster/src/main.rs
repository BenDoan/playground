extern crate atom_syndication;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::error::Error;
use std::io::{self, Write};
use std::str;
use std::string::String;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn main() {
    println!("Hello, world!");
    request_feed_xml().unwrap();
}

fn request_feed_xml() -> Result<String, Box<Error>> {
	let mut core = Core::new()?;
	let client = Client::new(&core.handle());

	let uri = "http://www.youtube.com/feeds/videos.xml?channel_id=UCckETVOT59aYw80B36aP9vw".parse()?;

	let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(|body| {
            let s = String::from_utf8_lossy(&body);
            futures::future::ok(s)
        })
    });

    println!("{:?}", core.run(work)?);

	Ok("".into())
}
