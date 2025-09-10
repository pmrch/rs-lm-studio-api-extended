/// This module contains the main embedding logic and API client for LM Studio's embedding endpoint.
/// It provides the structures and methods for sending requests and receiving embedding vectors.
use crate::prelude::*;
use super::*;

use anyhow::Ok;
use reqwest::{Client};

#[derive(Debug)]
pub enum EmbeddingResult {
    Single(Vec<f32>),
    Multi(Vec<Vec<f32>>)
}

impl EmbeddingResult {
    pub fn len(&self) -> usize {
        match self {
            EmbeddingResult::Single(v) => v.len(),
            EmbeddingResult::Multi(vv) => vv.len()
        }
    }
}

/// Embedding client for LM Studio API
pub struct Embedder {
    /// API endpoint URL for embedding requests
    pub url: String,

    /// HTTP client used to send requests
    pub client: Client
}

impl Embedder {
    /// Create a new Embedding client for a given port. 
    /// This sets up the endpoint and HTTP client.
    pub fn new(url: Option<String>) -> 
    Self 
    {
        if url.is_some() {
            Self {
                url: url.unwrap().to_string(),
                client: Client::new()
            }
        } else { 
            Self {
                url: fmt!("http://127.0.0.1:1234/v1/embeddings"),
                client: Client::new()
            }
        }
    }

    /// Send an embedding request and return the response from the API.
    /// This method serializes the request, sends it, and parses the response.
    pub async fn send(&mut self, req: EmbeddingRequest) -> anyhow::Result<EmbeddingData> {
        let resp = self.client.post(&self.url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(resp)
    }

    /// Send an embedding request and return the response from the API.
    /// This method uses the send() method, and returns the actual embedding vector
    pub async fn embed(&mut self, req: EmbeddingRequest) -> anyhow::Result<EmbeddingResult> {
        if !self.is_batch(req.clone()).await? {
            let embedding = self.single_embed(req).await?;
            Ok(EmbeddingResult::Single(embedding))
        } else {
            let embeddings = self.batch_embed(req).await?;
            Ok(EmbeddingResult::Multi(embeddings))
        }
    }

    async fn single_embed(&mut self, req: EmbeddingRequest) -> anyhow::Result<Vec<f32>> {
        let resp = self.send(req).await?;
        let result = resp.data[0].actual_embedding();

        Ok(result)
    }

    async fn batch_embed(&mut self, req: EmbeddingRequest) -> anyhow::Result<Vec<Vec<f32>>> {
        let resp = self.send(req).await?;
        let mut multi_embed: Vec<Vec<f32>> = Vec::new();

        for embed in resp.data {
            let actual_embed = embed.embedding;
            multi_embed.push(actual_embed);
        }

        Ok(multi_embed)
    }

    async fn is_batch(&mut self, req: EmbeddingRequest) -> anyhow::Result<bool> {
        Ok(self.send(req).await?.data.len() > 1)
    }

}