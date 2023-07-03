use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ActionResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub error: String,
}
