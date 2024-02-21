use crate::json_serialization::to_do_items::ToDoItems;
use actix_web::Responder;

pub async fn get() -> impl Responder {
    return ToDoItems::get_state();
}

// use actix_web::{web, Responder};
// use serde_json::value::Value;
// use serde_json::Map;

// use crate::json_serialization::to_do_items::ToDoItems;
// use crate::state::read_file;
// // crate refers to current app's root - there I got src/state.rs within which there is read_file fn;
// use crate::to_do::{enums::TaskStatus, to_do_factory, ItemTypes};

// pub async fn get() -> impl Responder {
//     let state: Map<String, Value> = read_file("./state.json");

//     let mut array_buffer = Vec::new();

//     for (key, value) in state {
//         let status = TaskStatus::from_string(value.as_str().unwrap().to_string()); // valueborrowsed?
//         let item: ItemTypes = to_do_factory(&key, status);
//         array_buffer.push(item);
//     }
//     let return_package: ToDoItems = ToDoItems::new(array_buffer);

//     return web::Json(return_package);
// }

// // pub async fn get() -> Map<String, Value> {
// //     let state: Map<String, Value> = read_file("./state.json");
// //     return state;
// // }
