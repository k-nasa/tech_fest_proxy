#![feature(async_closure)]

use serde::{Deserialize, Serialize};
use http::header::HeaderValue;
use tide::middleware::{CorsOrigin, CorsMiddleware};

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
    circle_cut_image: Option<Image>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Image{
    url: String,
}

async fn circles(_: tide::Context<()>) -> tide::EndpointResult {
    const JSON: &str = include_str!("assets/circles.json");

    let data: serde_json::Value = serde_json::from_str(JSON).unwrap();
    let circles: Circles = serde_json::from_value(data.clone()).unwrap();


    Ok(tide::response::json(circles))
}

#[derive(Debug, Serialize, Deserialize)]
struct Products {
    list: Option<Vec<Product>>
}

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    id: String,
    name: String,
    description: String,
    images: Option<Vec<Image>>,
}

async fn product(cx: tide::Context<()>) -> tide::EndpointResult {
    let id: String = cx.param("id").unwrap();

    let url = format!("https://techbookfest.org/api/product?circleExhibitInfoID={}&limit=100", id);

    let mut res = surf::get(url).await.unwrap();
    let json_string = res.body_string().await.unwrap();
    let data: serde_json::Value = serde_json::from_str(&json_string).unwrap();
    let products: Products = serde_json::from_value(data.clone()).unwrap();

    Ok(tide::response::json(products))
}


fn main() {
    let mut app = tide::App::new();

    let cors = CorsMiddleware::new()
        .allow_origin(CorsOrigin::Any)
        .allow_methods(HeaderValue::from_static("GET"));

    app.middleware(cors);
    app.at("/").get(|_| async move { "hello world"});

    // FIXME refactor to nested route
    app.at("/api").nest(|api| {
        api.at("/circles").get(circles);
        api.at("/product/:id").get(product);
    });

    app.run("127.0.0.1:8080").unwrap();
}
