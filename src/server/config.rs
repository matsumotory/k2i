use rustc_serialize::json;
use std::path::PathBuf;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ServerConfiguration {
    hostname: String,
    port: i32,
    pub threads: usize,
    https: bool,
    pub tls: Option<TLSConfiguration>,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct TLSConfiguration {
    pub cert: PathBuf,
    pub key: PathBuf,
}

impl ServerConfiguration {
    pub fn new() -> ServerConfiguration {
        ServerConfiguration {
            hostname: "127.0.0.1".to_string(),
            port: 3000,
            threads: 1,
            https: false,
            tls: None,
        }
    }

    pub fn hostname(&mut self, hostname: &'static str) -> &mut ServerConfiguration {
        self.hostname = hostname.to_string();
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

    pub fn threads_auto(&mut self) -> &mut ServerConfiguration {
        self.threads(num_cpus::get());
        self
    }

    pub fn hostport(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    pub fn tls_cert_key(&mut self, cert: PathBuf, key: PathBuf) -> &mut ServerConfiguration {
        let tls = Some(TLSConfiguration {
            cert: cert,
            key: key,
        });
        self.https = true;
        self.tls = tls;
        self
    }

    pub fn finalize(&self) -> ServerConfiguration {
        ServerConfiguration {
            hostname: self.hostname.clone(),
            port: self.port,
            threads: self.threads,
            https: self.https,
            tls: self.tls.clone(),
        }
    }

    pub fn dump(&self) -> String {
        json::encode(self).unwrap()
    }
}
