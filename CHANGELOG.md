# Changelog
All changes to this project will be documented in this file

## [0.1.0]
### Added
- Support for text embeddings, only supports non-batched requests
- Some minimal in-code documentation
- Code examples

### Fixed 
- The Cargo.toml to prevent panic upon build due to the macron dependency

## [0.1.1]
### Added
- Implemented handling of batched requests for text embeddings

## [0.1.2]
### Added
- A method to the Context struct of chat that allows you to insert context 
from RAG into the system prompt
- A method to Chat that allows changing the URL of your LM Studio server

## [0.1.3]
### What's New
- Default Chat setup: Chat now initializes automatically on port 1234, no 
need to specify it.
- Embedder improvements: The Embedding struct is now called Embedder for clarity.
- Easier Context handling: The Context struct’s fields are now public, and a default 
implementation is included.
- More flexible Role enum: You can now create a Role from a string using try_from().
- Enhanced EmbeddingInput:
	* Can be created from a string, string slice, Vec<String>, Vec<&str>, or a slice 
	of string slices (&[&str]).
	* Supports serialization via serde.

- Additional Model option: Another choice is now available in the Model enum.