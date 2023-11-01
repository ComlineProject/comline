// Relative Modules
mod package;
mod template;

// Standard Uses

// Crate Uses
use crate::web_ui::package::package_show;

// External Uses
use axum::Router;
use axum::routing::get;
use axum::response::Html;


pub fn register_routes(router: Router) -> Router {
    router
        .route("/", get(root))
        .route("/packages", get(packages))
        .route("/package/:name", get(package_show))
}

async fn root() -> Html<&'static str> {
    Html(include_str!("../../templates/index.html"))
}

async fn packages() -> Html<&'static str> {
    Html(include_str!("../../templates/packages.html"))
}
