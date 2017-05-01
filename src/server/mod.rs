use std::time::Duration;

use iron::prelude::*;
use iron::status;
use iron::Timeouts;

mod response;
pub mod config;

use self::config::ServerConfiguration;

pub struct K2I {
    config: ServerConfiguration,
}

impl K2I {
    pub fn new(config: ServerConfiguration) -> K2I {
        K2I { config: config }
    }

    pub fn run(&self) {
        let mut server = Iron::new(hello_world);

        server.threads = self.config.threads;
        server.timeouts = Timeouts {
            keep_alive: Some(Duration::from_secs(10)),
            read: Some(Duration::from_secs(10)),
            write: Some(Duration::from_secs(10)),
        };

        let _listening = server.http(self.config.hostport()).unwrap();
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, response::hello())))
}
