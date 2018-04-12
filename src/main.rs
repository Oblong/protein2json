extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
use std::io;

fn main() {
    let deserialized : serde_yaml::Mapping = serde_yaml::from_reader(io::stdin()).unwrap();
    serde_json::to_writer_pretty(io::stdout(), &deserialized).unwrap();
}
