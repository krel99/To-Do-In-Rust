use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
    let file = std::fs::File::open("./build_config.yml").unwrap();
    let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();
    let version = map.get("ALLOWED_VERSION").unwrap().as_str().unwrap();
    let mut f = File::create("./src/output_data.txt").unwrap();
    write!(f, "{}", version).unwrap();
}
