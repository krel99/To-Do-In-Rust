use actix_web::web;
mod content_loader;
mod items;

pub fn app_views_factory(app: &mut web::ServiceConfig) {
    app.route("/", web::get().to(items::items));
}
