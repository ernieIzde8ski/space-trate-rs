use serde::{de, Deserialize, Serialize};

// generic api response stuff
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorData {
    symbol: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    message: String,
    code: i32,
    data: Option<ErrorData>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            Some(data) => {
                write!(
                    f,
                    "SpaceTraders API error (code={}): {}\nAdditional info: {}",
                    self.code,
                    self.message,
                    data.symbol.join(" ")
                )
            }
            None => {
                write!(
                    f,
                    "SpaceTraders API error (code={}): {}",
                    self.code, self.message
                )
            }
        }
    }
}
impl std::error::Error for ApiError {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ApiResult<T> {
    Data(T),
    Error(ApiError),
}

impl<T> ApiResult<T> {
    pub fn to_result(self) -> Result<T, Box<dyn std::error::Error>> {
        match self {
            Self::Data(resp) => Ok(resp),
            Self::Error(err) => Err(Box::new(err)),
        }
    }
}

// specific api stuff

#[derive(Serialize, Deserialize, Debug)]
pub struct RegistryResponse {
    pub token: String,
}
// functions
pub fn decode_response<T: de::DeserializeOwned>(s: &str) -> Result<T, Box<dyn std::error::Error>> {
    serde_json::from_str::<ApiResult<T>>(s)?.to_result()
}
