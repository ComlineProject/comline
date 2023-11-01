// Standard Uses

// Crate Uses
use crate::storage;
use crate::storage::projects::ProjectInfo;
use crate::web_ui::template::HtmlTemplate;

// External Uses
use axum::extract;
use axum::response::IntoResponse;
use askama::Template;


#[derive(Template)]
#[template(path = "package.html")]
struct PackageTemplate {
    name: String,
    info: Option<ProjectInfo>
}


pub async fn package_show(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let info = storage::projects::find_project(name.clone());

    let template = PackageTemplate { name, info, };

    HtmlTemplate(template)
}
