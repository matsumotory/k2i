extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate procps_sys;

mod server;

use server::K2I;
use server::config::ServerConfiguration;

fn main() {

    let config = ServerConfiguration::new()
        .hostname("127.0.0.1")
        .port(8000)
        .threads(4)
        .finalize();

    println!("On {}", config.hostport());

    let _server = K2I::new(config).run();
}
