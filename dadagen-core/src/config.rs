use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct DadagenConfig {
    pub lists: HashMap<String, ListData>,
}

#[derive(Debug, Deserialize)]
pub struct ListData {
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub values: Option<HashMap<String, Vec<String>>>,
}