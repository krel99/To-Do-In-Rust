use actix_web::HttpRequest;
use serde_json::value::Value;
use serde_json::Map;

use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory};

pub async fn create(req: HttpRequest) -> String {
    let state: Map<String, Value> = read_file("./state.json"); // define what type we are reading from json
    let title: String = req.match_info().get("title").unwrap().to_string(); // get the title from the URL (request)
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING); // create a new to_do item
    process_input(item, "create".to_string(), &state);
    return format!("{} created", title); // return a message
}
