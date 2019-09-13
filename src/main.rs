#![feature(async_closure)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Circles {
    pub list: Vec<Circle>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
struct Circle {
    pub id: String,
    pub name: String,
    pub name_ruby: String,
    pub pen_name: String,
    pub genre: String,
    pub genre_free_format: String,
    pub circle_cut_image: Option<CircleCutImage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CircleCutImage {
    url: String,
}

async fn circles(_: tide::Context<()>) -> tide::EndpointResult {
    let mut res = surf::get("https://techbookfest.org/api/circle?eventID=tbf07&limit=642").await.unwrap();

    let json_string = res.body_string().await.unwrap();
    let data: serde_json::Value = serde_json::from_str(&json_string).unwrap();
    let circles: Circles = serde_json::from_value(data.clone()).unwrap();


    Ok(tide::response::json(circles))
}

fn main() {
    let mut app = tide::App::new();

    app.at("/api/circles").get(circles);
    app.run("127.0.0.1:8080").unwrap();
}
