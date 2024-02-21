mod create;
mod edit;
mod get;

use actix_web::web::{get, post, scope, ServiceConfig};

// note: create is not public, instead it is used here to create a route
pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
            .route("get", get().to(get::get)) // define view and URL
            .route("edit", post().to(edit::edit)),
    );
}
