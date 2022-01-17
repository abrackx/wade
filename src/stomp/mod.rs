use std::iter::Map;

mod client;
mod server;

struct StompFrame<T> {
    pub command: T,
    pub headers: Map<String, String>,
    pub body: String,
}