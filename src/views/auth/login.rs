use crate::diesel;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

use crate::database::DB;
use crate::json_serialization::login::Login;
use crate::jwt::JwToken;
use crate::models::user::user::User;
use crate::schema::users;

pub async fn login(credentials: web::Json<Login>, db: DB) -> impl Responder {
    let password = credentials.password.clone();
    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&db.connection)
        .unwrap();
    if users.len() == 0 {
        return HttpResponse::NotFound();
    }
    if users.len() > 1 {
        return HttpResponse::Conflict();
    }
    match users[0].verify(password) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            HttpResponse::Ok()
                .append_header(("token", raw_token))
                .take()
        }
        false => HttpResponse::Unauthorized(),
    }
}
