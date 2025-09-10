// The Input struct represents a single input message or text 
// for embedding.
// It is used to encapsulate the content that will be processed 
// by the embedding API.
use crate::prelude::*;

// A message or text to be embedded
#[derive(Debug, Clone, From, Deserialize)]
pub enum EmbeddingInput {
    Sentence(String),
    Sentences(Vec<String>)
}

/*
The reason for making it an enum is to easily support 
different input types now and allow for easy future extension.
*/

impl From<&str> for EmbeddingInput {
    fn from(value: &str) -> Self {
        EmbeddingInput::Sentence(value.to_string())
    }
}

impl From<String> for EmbeddingInput {
    fn from(value: String) -> Self {
        EmbeddingInput::Sentence(value)
    }
}

impl From<Vec<String>> for EmbeddingInput {
    fn from(value: Vec<String>) -> Self {
        EmbeddingInput::Sentences(value)       
    }
}

impl From<Vec<&str>> for EmbeddingInput {
    fn from(value: Vec<&str>) -> Self {
        let converted = value.into_iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>();

        EmbeddingInput::Sentences(converted)
    }
}

impl From<&[&str]> for EmbeddingInput {
    fn from(value: &[&str]) -> Self {
        let converted = value
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>();

        EmbeddingInput::Sentences(converted)
    }
}

impl Serialize for EmbeddingInput {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        
        match self {
            EmbeddingInput::Sentence(text) => serializer.serialize_str(text),
            EmbeddingInput::Sentences(vec) => vec.serialize(serializer)
        }
    }
}