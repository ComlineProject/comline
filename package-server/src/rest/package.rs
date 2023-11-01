// Standard Uses

// Crate Uses
use crate::storage::FROZEN_PROJECT_RELATIVE_DIR;
use crate::rest::web_address;

// External Uses
use axum::{extract, http::StatusCode, Router};
use axum::routing::get;
use serde::Deserialize;


/*
pub async fn find_package_url(Json(payload): Json<PackageMeta>) -> (StatusCode, Json<String>) {
    let package_url = "".to_owned();

    (StatusCode::CREATED, Json(package_url))
}
*/

#[allow(unused)]
pub async fn find_package_url(
    extract::Path((name, version)): extract::Path<(String, String)>
) -> (StatusCode, String) {
    let url = format!(
        "{}/{}/{}{}",
        web_address(), FROZEN_PROJECT_RELATIVE_DIR, name, version
    );

    (StatusCode::OK, url)
}


#[allow(unused)]
#[derive(Deserialize)]
pub struct PackageMeta {
    name: String, version: String
}

pub fn register_routes(router: Router) -> Router {
    router
        .route("/api/package/:name/:version", get(find_package_url))
}
