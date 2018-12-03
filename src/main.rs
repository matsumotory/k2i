extern crate iron;
extern crate procps_sys;
extern crate router;
extern crate rustc_serialize;
extern crate num_cpus;

mod server;

use server::config::ServerConfiguration;
use server::K2I;

fn main() {
    let config = ServerConfiguration::new()
        .hostname("127.0.0.1")
        .port(8000)
        // for now catch segfault procps-sys when multi-threading
        .threads_auto()
        .finalize();

    println!("On {}", config.hostport());

    let _server = K2I::new(config).run();
}
