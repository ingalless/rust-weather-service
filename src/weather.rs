use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Weather {
    pub rainfall: String,
    pub clouds: String,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherResponse {
    pub current: Weather,
    pub next: Weather,
    pub interval: i32,
    pub last_incremented: u64,
}
