use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use crate::json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems};
use crate::jwt::JwToken;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory};

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");

    let status: TaskStatus;
    match &state.get(&to_do_item.title) {
        Some(result) => {
            status = TaskStatus::from_string(result.as_str().unwrap().to_string());
        }
        None => {
            return HttpResponse::NotFound()
                .json(format!("{} not in the state", &to_do_item.title));
        }
    }

    let existing_item = to_do_factory(to_do_item.title.as_str(), status.clone());
    process_input(existing_item, "delete".to_owned(), &state);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
