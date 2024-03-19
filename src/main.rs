mod fonts;
mod highlight;
mod image;

use axum::{extract::Query, http::header, response::IntoResponse, routing::get, Router};
use catppuccin::{ColorName, FlavorName};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    lazy_static::initialize(&fonts::FONT_REGULAR);
    lazy_static::initialize(&fonts::FONT_ITALIC);
    lazy_static::initialize(&fonts::FONT_BOLD_REGULAR);
    lazy_static::initialize(&fonts::FONT_BOLD_ITALIC);

    let app = Router::new().route("/", get(root)).route(
        "/style.css",
        get(|| async {
            let css = include_str!("../web/dist/style.css");
            ([(header::CONTENT_TYPE, "text/css")], css)
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, rename = "t")]
    text: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "f", default = "default_flavor")]
    flavor: FlavorName,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "c", default = "default_color")]
    color: ColorName,
}

const fn default_flavor() -> FlavorName {
    FlavorName::Frappe
}

const fn default_color() -> ColorName {
    ColorName::Pink
}

async fn root(Query(params): Query<Params>) -> axum::response::Result<impl IntoResponse> {
    if params.text.is_some() {
        let img = crate::image::create(params.text.unwrap().as_str(), params.flavor, params.color);

        let mut write_buf = Vec::new();
        img.encode(ril::ImageFormat::Png, &mut write_buf).unwrap();

        let headers = [(header::CONTENT_TYPE, "image/png")];

        Ok((headers, write_buf).into_response())
    } else {
        let index = include_str!("../web/dist/index.html");
        Ok(axum::response::Html(index).into_response())
    }
}
