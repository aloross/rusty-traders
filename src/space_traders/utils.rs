use std::env;

use askama_axum::IntoResponse;
use axum::Json;
use dotenv::dotenv;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    agents: i32,
    ships: i32,
    systems: i32,
    waypoints: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ServerResets {
    next: String,
    frequency: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Announcements {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Links {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MostSubmittedChart {
    agent_symbol: String,
    chart_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MostCredits {
    agent_symbol: String,
    credits: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Leaderboards {
    most_credits: Vec<MostCredits>,
    most_submitted_charts: Vec<MostSubmittedChart>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Status {
    status: String,
    version: String,
    reset_date: String,
    description: String,
    stats: Stats,
    leaderboards: Leaderboards,
    server_resets: ServerResets,
    announcements: Vec<Announcements>,
    links: Vec<Links>,
}

pub async fn get_status() -> Result<impl IntoResponse, StatusCode> {
    let response = match reqwest::get("https://api.spacetraders.io/v2/").await {
        Ok(res) => res,
        Err(err) => {
            println!("{:#?}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    match response.json::<Status>().await {
        Ok(body) => Ok(Json(body)),
        Err(err) => {
            println!("{:#?}", err);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub fn get_token() -> String {
    dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN is not set");
    "Bearer ".to_owned() + &token
}
