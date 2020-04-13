mod config;
mod weather;

use crate::weather::*;
use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use std::ops::Add;
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct AppState {
    interval: Mutex<i32>,
    last_incremented: Mutex<Instant>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> Result<HttpResponse> {
    let interval: i32 = *data.interval.lock().unwrap();
    let mut last_incremented = data.last_incremented.lock().unwrap();
    let increment_at = last_incremented.add(Duration::from_secs(interval as u64));
    println!("increment {:?}", increment_at);
    if Instant::now()
        .checked_duration_since(increment_at)
        .is_none()
    {
        *last_incremented = Instant::now();
        // Change weather which means change current to next
        // and generate some new "random" weather
        // and probably make a new "random" interval
    }
    Ok(HttpResponse::Ok().json(WeatherResponse {
        current: Weather {
            rainfall: String::from("medium"),
            clouds: String::from("cloudy"),
        },
        next: Weather {
            rainfall: String::from("none"),
            clouds: String::from("clear"),
        },
        interval,
        last_incremented: last_incremented.elapsed().as_secs(),
    }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    let server = format!("{}:{}", config.server.host, config.server.port);
    println!("Starting server at http://{}", server);

    let app = web::Data::new(AppState {
        interval: Mutex::new(config.app.interval),
        last_incremented: Mutex::new(Instant::now()),
    });

    HttpServer::new(move || App::new().app_data(app.clone()).service(index))
        .bind(server)?
        .run()
        .await
}
