pub fn hello() -> &'static str {
    "mod hello world!"
}

pub mod sub_hello {
    pub fn hello() -> &'static str {
        "mod sub_hello world!"
    }
}
