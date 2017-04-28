pub struct ServerConfiguration {
  hostname: String,
  port: i32,
}

impl ServerConfiguration {
  pub fn new() -> ServerConfiguration {
    ServerConfiguration {
      hostname: "127.0.0.1".to_string(),
      port: 3000,
    }
  }

  pub fn hostname(&mut self, hostname: String) -> &mut ServerConfiguration {
    self.hostname = hostname;
    self
  }

  pub fn port(&mut self, port: i32) -> &mut ServerConfiguration {
    self.port = port;
    self
  }

  pub fn hostport(&mut self) -> String {
    format!("{}:{}", self.hostname, self.port)
  }
}
