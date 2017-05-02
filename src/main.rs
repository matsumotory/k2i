extern crate iron;
extern crate router;

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
