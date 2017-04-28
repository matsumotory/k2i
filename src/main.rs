extern crate iron;

use iron::prelude::*;
use iron::status;

mod hello;
mod config;

use config::ServerConfiguration;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, hello::hello())))
    }

    let mut config = ServerConfiguration::new();
    let _server = Iron::new(hello_world).http(config.hostport()).unwrap();
    println!("On {}", config.hostport());
}

