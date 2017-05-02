#[derive(Debug)]
pub struct ServerConfiguration {
    hostname: &'static str,
    port: i32,
    pub threads: usize,
}

impl ServerConfiguration {
    pub fn new() -> ServerConfiguration {
        ServerConfiguration {
            hostname: "127.0.0.1",
            port: 3000,
            threads: 1,
        }
    }

    pub fn hostname(&mut self, hostname: &'static str) -> &mut ServerConfiguration {
        self.hostname = hostname;
        self
    }

    pub fn port(&mut self, port: i32) -> &mut ServerConfiguration {
        self.port = port;
        self
    }

    pub fn threads(&mut self, threads: usize) -> &mut ServerConfiguration {
        self.threads = threads;
        self
    }

    pub fn hostport(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    pub fn finalize(&self) -> ServerConfiguration {
        ServerConfiguration {
            hostname: self.hostname,
            port: self.port,
            threads: self.threads,
        }
    }

    pub fn dump(&self) -> String {
        format!("K2I configuration: {:?}", self)
    }
}
