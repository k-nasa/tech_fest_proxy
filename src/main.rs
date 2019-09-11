#![feature(async_closure)]

async fn circles(_: tide::Context<()>) -> tide::EndpointResult {
    Ok(tide::response::json(""))
}

fn main() {
    let mut app = tide::App::new();

    app.at("/api/circles").get(circles);
    app.run("127.0.0.1:8080").unwrap();
}
