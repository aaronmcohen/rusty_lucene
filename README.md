# rusty_lucene

 [![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://img.shields.io/badge/version-0.1.0-blue)
 [![License](https://img.shields.io/badge/license-MIT-lightgrey)](https://img.shields.io/badge/license-MIT-lightgrey)
 [![Rust](https://img.shields.io/badge/rust-1.75-orange)](https://img.shields.io/badge/rust-1.75-orange)

 rusty_lucene is a Rust library that provides Lucene-style full-text search capabilities for applications. It offers efficient indexing and querying functionality with a clean, ergonomic API.

 ## Quick Start

 ### Prerequisites

 - Rust 1.75 or later
 - cargo (Rust package manager)

 ### Installation

 Add this to your `Cargo.toml`:

 ```toml
 [dependencies]
 rusty_lucene = "0.1.0"
 ```

 ### Basic Usage

 ```rust
 use rusty_lucene::IndexBuilder;

 fn main() {
     // Create an in-memory index
     let index = IndexBuilder::new().build_in_memory();

     // Add a document
     index.add_document(doc! {
         "title" => "Hello World",
         "content" => "This is a sample document"
     });

     // Search for documents
     let results = index.search("hello", 10);

     println!("Found {} matching documents", results.len());
 }
 ```

 ## Features

 - **Full-text search**: Efficient text search with ranking
 - **In-memory and disk-backed indexes**: Choose based on your needs
 - **Simple document API**: Easy document creation and manipulation
 - **Query DSL**: Expressive query building
 - **Highlighting**: Extract and highlight search terms in results

 ## Documentation

 - [Full API Documentation](docs/README.md)
 - [Query Guide](docs/querying.md)
 - [Indexing Guide](docs/indexing.md)
 - [Examples](examples/)

 ## Contributing

 See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to this project.

 ## License

 This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Version Inquiry

 To determine which version of rusty_lucene you are using:

 - **From Cargo.toml**: Check the `version` field in your project's `Cargo.toml`
 - **From source code**: Run `cargo tree | grep rusty_lucene` to see the resolved version
 - **From documentation**: The version is displayed in the build status badge at the top of this file (e.g., `version-0.1.0`)

 The current major.minor.patch version is **0.1.0**.
