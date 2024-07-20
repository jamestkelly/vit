pub mod handler;
pub mod model;
pub mod response;
pub mod route;

use serde::{ Deserialize, Serialize };
use warp::{ reject::Rejection, Filter };

use rust_embed::RustEmbed;

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    name: String,
}

#[derive(RustEmbed)]
#[folder = "web/build"]
struct Static;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let greet = warp::path!("api" / "greet")
        .and(warp::query::<Query>())
        .map(|q: Query| {
            log::debug!("/vit/greet {:?}", &q);
            format!("Hello, {}!", q.name)
        });

    let health_checker = warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handler::health_checker_handler);

    let static_files = warp::path("vit").and(warp::get()).and(warp_embed::embed(&Static)).boxed();

    let cors = warp
        ::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    let routes = static_files.with(cors).with(warp::log("api")).or(greet).or(health_checker);

    println!("🚀 Starting Vit server...");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
