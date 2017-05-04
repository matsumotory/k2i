use iron::Timeouts;

use iron::prelude::*;
use iron::status;
use procps_sys::readproc;
use router::Router;
use std::time::Duration;

pub mod config;
mod response;
mod procps;

use self::config::ServerConfiguration;
use self::procps::*;

pub struct K2I {
    config: ServerConfiguration,
    end_point: &'static str,
}

impl K2I {
    pub fn new(c: ServerConfiguration) -> K2I {
        K2I {
            config: c,
            end_point: "/k2i/api/v1",
        }
    }

    pub fn run(&self) {
        let mut router = Router::new();
        let config_dump = self.config.dump();

        // routing API
        router.get(format!("{}{}", self.end_point, "/"), root_response, "root");
        router.get(format!("{}{}", self.end_point, "/hello"),
                   hello_response,
                   "hello");
        router.get(format!("{}{}", self.end_point, "/config"),
                   move |_: &mut Request| Ok(Response::with((status::Ok, config_dump.as_str()))),
                   "config");
        router.get(format!("{}{}", self.end_point, "/proc"),
                   procps_response,
                   "procps");

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

fn procps_response(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok,
                       procps_json_encode(readproc::PROC_FILLCOM | readproc::PROC_FILLSTAT |
                                          readproc::PROC_FILLMEM |
                                          readproc::PROC_FILLARG |
                                          readproc::PROC_FILLCGROUP,
                                          0))))
}
