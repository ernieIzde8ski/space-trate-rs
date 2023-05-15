use reqwest;
pub mod responses;
pub mod schemas;
use responses::{ApiResult, RegistryResponse};

#[derive(Debug)]
pub struct SpaceTraderAgent {
    #[allow(dead_code)]
    client: reqwest::Client,
    token: String,
}

impl SpaceTraderAgent {
    pub fn new(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    #[tokio::main]
    pub async fn register(
        symbol: &str,
        faction: Option<&str>,
        client: Option<reqwest::Client>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        //! symbol should be a unique alphanumeric identifier, otherwise known as an agent
        //!
        //! faction can be any of "COSMIC", "VOID", "GALACTIC", or "QUANTUM" at the time
        //! of writing. COSMIC is said to be the default faction
        let client = client.unwrap_or_else(reqwest::Client::new);
        let params: [(&str, &str); 2] =
            [("symbol", symbol), ("faction", faction.unwrap_or("COSMIC"))];
        let result = client
            .post("https://api.spacetraders.io/v2/register")
            .header("Content-Type", "application/json")
            .form(&params)
            .send()
            .await?
            .json::<ApiResult<i32>>()
            .await?
            .to_result()?;

        Ok(SpaceTraderAgent {
            client: client,
            token: "result.token".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    mod schema;
}
