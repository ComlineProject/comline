// Relative Modules
mod user;
mod publish;
mod package;

// Standard Uses
use std::net::SocketAddr;

// Crate Uses
use crate::web_ui;

// External Uses
use axum::Router;
use axum::routing::post;
use once_cell::sync::OnceCell;


pub static ADDRESS: OnceCell<String> = OnceCell::new();
pub fn web_address() -> String {
    format!("http://{}", ADDRESS.get().unwrap())
}

pub async fn init() {
    let app = Router::new();
    let app = register_routes(app);
    let app = web_ui::register_routes(app);
    let app = package::register_routes(app);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    ADDRESS.get_or_init(|| addr.to_string());

    tracing::info!("listening on http://{}/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn register_routes(router: Router) -> Router {
    router
        .route("/api/users/register", post(user::create_user))
        .route("/api/publish/package", post(publish::publish_package))
}
