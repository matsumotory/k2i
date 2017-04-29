extern crate iron;

use std::time::Duration;

use iron::prelude::*;
use iron::status;
use iron::Timeouts;

mod response;
mod config;

use config::ServerConfiguration;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, response::hello())))
    }

    let config = ServerConfiguration::new()
        .hostname("127.0.0.1")
        .port(8000)
        .threads(4)
        .finalize();

    let mut server = Iron::new(hello_world);

    server.threads = config.threads;
    server.timeouts = Timeouts {
        keep_alive: Some(Duration::from_secs(10)),
        read: Some(Duration::from_secs(10)),
        write: Some(Duration::from_secs(10)),
    };

    println!("On {}", config.hostport());

    server.http(config.hostport()).unwrap();

}
