use serde::Deserialize;

#[derive(Deserialize)]
pub struct CodeReq {
    pub language: Option<String>,
    pub content: String,
}
