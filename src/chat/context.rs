// use crate::prelude::*;
use super::{ Role, Message };

// Chat context
#[derive(Debug, Clone)]
pub struct Context {
    pub messages: Vec<Message>,
    pub context_tokens: usize,
    pub context_limit: usize
}

impl Default for Context {
    fn default() -> Self {
        Context { 
            messages: vec![
                Message { 
                    role: Role::System, 
                    content: "You are a helpful, knowledgeable, and friendly assistant.".to_string()
                }
            ], 
            context_tokens: 0, 
            context_limit: 4090 
        }
    }
}

impl Context {
    // Creates a new chat context
    pub fn new<S>(context: S, context_limit: usize) -> Self
    where
        S: Into<String>
    {
        Self {
            messages: vec![Message::new(Role::System, context.into())],
            context_tokens: 0,
            context_limit
        }
    }

    // Adds context from RAG to system prompt
    pub fn edit<S>(&mut self, modification: S)
    where 
        S: Into<String>
    {
        let sysprompt_extra = format!("\n\nContext: [\n\t{}]", modification.into());
        self.messages[0].content.push_str(&sysprompt_extra);
    }

    // Add a message to context
    pub fn add<M>(&mut self, message: M)
    where
        M: Into<Message>
    {
        let message = message.into();

        // add message to context:
        self.context_tokens += message.content.chars().count();
        self.messages.push(message);

        // remove old extra messages:
        while self.messages.len() > 2 && self.context_tokens > self.context_limit {
            self.context_tokens -= self.messages[1].content.chars().count();
            self.messages.remove(1);
        }
    }

    // Get all context messages
    pub fn get(&self) -> Vec<Message> {
        self.messages.clone()
    }

    // Clear context messages
    pub fn clear(&mut self) {
        self.messages.truncate(1);
    }
}
