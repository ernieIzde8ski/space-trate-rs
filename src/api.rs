use serde::de;

pub mod responses;
pub mod results;
pub mod schema;

// generic api response stuff

pub fn decode_api_response<T: de::DeserializeOwned>(
    s: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    serde_json::from_str::<results::ApiResult<T>>(s)?.to_result()
}

pub struct SpaceTraderApi {
    token: String,
    client: reqwest::Client,
}

impl SpaceTraderApi {}
