use std::time::Duration;

use iron::prelude::*;
use iron::status;
use iron::Timeouts;

use router::Router;

mod response;
pub mod config;

use self::config::ServerConfiguration;

pub struct K2I {
    config: ServerConfiguration,
}

impl K2I {
    pub fn new(c: ServerConfiguration) -> K2I {
        K2I { config: c }
    }

    pub fn run(&self) {
        let mut router = Router::new();
        router.get("/", root_response, "root");
        router.get("/hello", hello_response, "hello");

        let mut server = Iron::new(router);

        server.threads = self.config.threads;
        server.timeouts = Timeouts {
            keep_alive: Some(Duration::from_secs(10)),
            read: Some(Duration::from_secs(10)),
            write: Some(Duration::from_secs(10)),
        };

        let _listening = server.http(self.config.hostport()).unwrap();
    }
}

fn hello_response(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, response::hello())))
}

fn root_response(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, response::root())))
}
