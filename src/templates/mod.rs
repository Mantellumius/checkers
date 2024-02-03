use askama::Template;
use serde::Deserialize;

#[derive(Deserialize, Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
    pub title: String,
}
