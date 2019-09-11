#![feature(async_closure)]

async fn circles(_: tide::Context<()>) -> tide::EndpointResult {
    let mut res = surf::get("https://techbookfest.org/api/circle?eventID=tbf07&limit=642").await.unwrap();
    let json_string = res.body_string().await.unwrap();
    let data: serde_json::Value = serde_json::from_str(&json_string).unwrap();

    Ok(tide::response::json(data))
}

fn main() {
    let mut app = tide::App::new();

    app.at("/api/circles").get(circles);
    app.run("127.0.0.1:8080").unwrap();
}
