use askama_axum::IntoResponse;
use axum::Json;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;

use super::utils::get_token;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Agent {
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: i64,
    starting_faction: String,
    ship_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    data: Agent,
}

pub async fn get_agent() -> Result<impl IntoResponse, StatusCode> {
    println!("{}", get_token());
    let client = reqwest::Client::new();
    let response = match client
        .get("https://api.spacetraders.io/v2/my/agent")
        .header(reqwest::header::AUTHORIZATION, get_token())
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            println!("{:#?}", err);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    println!("{:#?}", response);

    match response.json::<Data>().await {
        Ok(body) => Ok(Json(body)),
        Err(err) => {
            println!("{:#?}", err);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
