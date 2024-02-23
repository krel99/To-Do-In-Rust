use actix_web::HttpRequest;
use actix_web::HttpResponse;
// use serde_json::value::Value;
use serde_json::Map;
use serde_json::Value;

use crate::json_serialization::to_do_items::ToDoItems;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory};

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json"); // define what type we are reading from json
    let title: String = req.match_info().get("title").unwrap().to_string(); // get the title from the URL (request)
    let item = to_do_factory(&title.as_str(), TaskStatus::PENDING); // create a new to_do item
    process_input(item, "create".to_string(), &state);
    return HttpResponse::Ok().json(ToDoItems::get_state());
}
