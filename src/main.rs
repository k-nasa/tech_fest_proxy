#![feature(async_closure)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Circles {
    list: Vec<Circle>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct Circle {
    id: String,
    name: String,
    name_ruby: String,
    pen_name: String,
    genre: String,
    genre_free_format: String,
    circle_cut_image: Option<CircleCutImage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CircleCutImage {
    url: String,
}

async fn circles(_: tide::Context<()>) -> tide::EndpointResult {
    const JSON: &str = include_str!("assets/circles.json");

    let data: serde_json::Value = serde_json::from_str(JSON).unwrap();
    let circles: Circles = serde_json::from_value(data.clone()).unwrap();


    Ok(tide::response::json(circles))
}

fn main() {
    let mut app = tide::App::new();

    app.at("/api/circles").get(circles);
    app.run("127.0.0.1:8080").unwrap();
}
