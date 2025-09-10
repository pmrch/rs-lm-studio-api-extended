/// EmbeddingRequest defines the structure for an 
/// embedding API request.
/// It specifies the model, input texts, and optional 
/// encoding format for the request.
use crate::{prelude::*, EmbeddingInput};
use super::{ EmbeddingModel };


#[derive(Debug, Clone, Serialize)]
pub struct EmbeddingRequest {
    /// Model to use for embedding. Determines which model 
    /// will process the input.
    pub model: EmbeddingModel,

    /// Input text(s) to embed. Can be a batch of texts for efficiency.
    pub input: EmbeddingInput,

    /// Optional encoding format (e.g., "float"). Controls the output format 
    /// of the embedding vector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
}

/// Provides a default EmbeddingRequest with empty input and default 
/// encoding format.
/// This is useful for initializing requests before populating fields.
impl ::std::default::Default for EmbeddingRequest {
    fn default() -> Self {
        Self {
            model: EmbeddingModel::Custom("".into()),
            input: EmbeddingInput::from("Rust is magic."),
            encoding_format: Some("float".to_string())
        }
    }
}

impl EmbeddingRequest {
    pub fn make_sendable(&self, input: EmbeddingInput) -> Vec<String> {
        match input {
            EmbeddingInput::Sentence(s) => vec![s],
            EmbeddingInput::Sentences(ss) => ss
        }
    }
}
