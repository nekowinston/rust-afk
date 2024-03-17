mod image;

use crate::image::create_image;
use axum::{extract::Query, http::header, response::IntoResponse, routing::get, Router};
use catppuccin::{ColorName, FlavorName};
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

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

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, rename = "t")]
    text: Option<String>,
    #[serde(default, rename = "i", deserialize_with = "parse_bool")]
    italic: bool,
    #[serde(
        default = "default_flavor",
        rename = "f",
        deserialize_with = "parse_flavor"
    )]
    flavor: Option<FlavorName>,
    #[serde(
        default = "default_color",
        rename = "c",
        deserialize_with = "parse_color"
    )]
    color: Option<ColorName>,
}

fn default_flavor() -> Option<FlavorName> {
    Some(FlavorName::Frappe)
}

fn default_color() -> Option<ColorName> {
    Some(ColorName::Pink)
}

fn parse_bool<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    match s.as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(de::Error::custom("expected true or false")),
    }
}

fn parse_flavor<'de, D>(de: D) -> Result<Option<FlavorName>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    FlavorName::from_str(&s)
        .map_err(de::Error::custom)
        .map(Some)
}

fn parse_color<'de, D>(de: D) -> Result<Option<ColorName>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    ColorName::from_str(&s).map_err(de::Error::custom).map(Some)
}

async fn root(Query(params): Query<Params>) -> axum::response::Result<impl IntoResponse> {
    if params.text.is_some() {
        let img = create_image(
            params.text.unwrap().as_str(),
            params.italic,
            params.flavor.unwrap(),
            params.color.unwrap(),
        );

        let mut write_buf = Vec::new();
        img.encode(ril::ImageFormat::Png, &mut write_buf).unwrap();

        let headers = [(header::CONTENT_TYPE, "image/png")];

        Ok((headers, write_buf).into_response())
    } else {
        let index = include_str!("../web/dist/index.html");
        Ok(axum::response::Html(index).into_response())
    }
}
