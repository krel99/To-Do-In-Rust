use crate::diesel;
use diesel::prelude::*;

use actix_web::HttpRequest;
use actix_web::HttpResponse;

use crate::database::DB;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwToken;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;

pub async fn create(token: JwToken, req: HttpRequest, db: DB) -> HttpResponse {
    let title: String = req.match_info().get("title").unwrap().to_string();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&db.connection);
    }
    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id));
}
