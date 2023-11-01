// Standard Uses

// Crate Uses

// External Uses
use axum::{http::StatusCode, Json};
use serde::Deserialize;


#[allow(unused)]
pub async fn publish_package(Json(payload): Json<PackageDetails>) -> (StatusCode, Json<String>) {
    let package_url = "".to_owned();

    (StatusCode::CREATED, Json(package_url))
}


#[allow(unused)]
#[derive(Deserialize)]
pub struct PackageDetails {
    repository_url: String
}

