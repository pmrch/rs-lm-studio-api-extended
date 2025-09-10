use core::fmt;
use std::str::FromStr;
use serde_json;

use crate::prelude::*;
use super::*;


#[derive(Debug)]
pub struct ParseEmbeddingDataError(String);

impl fmt::Display for ParseEmbeddingDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse EmbeddingData: {}", self.0)
    }
}

impl std::error::Error for ParseEmbeddingDataError {}


#[derive(Serialize, Deserialize, Clone)]
pub struct EmbeddingData {
    pub object: String,
    pub data: Vec<EmbeddingResponse>,
    pub model: String,
    pub usage: EmbeddingUsage
}

impl FromStr for EmbeddingData {
    type Err = ParseEmbeddingDataError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
            .map_err(|e: serde_json::Error| ParseEmbeddingDataError(e.to_string()))
    }
}

impl fmt::Display for EmbeddingData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(s) => write!(f, "{}", s),
            Err(e) => write!(f, "<failed to display EmbeddingData: {}>", e)
        }
    }
}