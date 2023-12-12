# Chapter: File System Context in Kalosm

## Introduction

Kalosm, a versatile language model, provides a powerful file system context that allows users to seamlessly integrate document management capabilities into their applications. This chapter will guide you through the process of utilizing the file system context in Kalosm, demonstrating its features using a practical example.

## Setting Up the Project

Before we delve into the details, make sure you have a Rust project initialized. Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
kalosm = "0.1"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
```

This example also utilizes the `tracing` and `tokio` libraries. Ensure you have them added to your project dependencies as shown.

## Initializing the File System Context

To leverage the file system context, we'll create a `DocumentFolder` that represents a folder containing documents. Add the necessary imports to your Rust file:

```rust
use kalosm::language::*;
use std::io::Write;
use std::path::PathBuf;
```

Now, let's initialize the file system context within the `main` function:

```rust
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize the document folder from a specified path
    let documents = DocumentFolder::try_from(PathBuf::from("./documents")).unwrap();
```

In this example, we assume that there's a folder named "documents" in the current directory. Adjust the path as needed for your project structure.

## Building Document Databases

Next, we'll create two document databases: one for exact matching and another for fuzzy searching. The `DocumentDatabase` and `FuzzySearchIndex` structures facilitate efficient querying of documents.

```rust
    let mut database = DocumentDatabase::new(
        Bert::builder().build().unwrap(),
        ChunkStrategy::Sentence {
            sentence_count: 1,
            overlap: 0,
        },
    );
    database.extend(documents.clone()).await.unwrap();

    let mut fuzzy = FuzzySearchIndex::default();
    fuzzy.extend(documents).await.unwrap();
```

The `DocumentDatabase` is configured with a `Bert` model for natural language processing. Adjust the parameters based on your requirements. The `FuzzySearchIndex` is initialized for fuzzy search capabilities.

## Resource-Augmented Generation

Kalosm supports resource-augmented generation, enhancing the language model's understanding of the context. The `extend` method is used to augment the model with documents from the specified folder. The `await` keyword is used to asynchronously perform this operation.

```rust
    database.extend(documents.clone()).await.unwrap();
    fuzzy.extend(documents).await.unwrap();
```

This step ensures that the language model is aware of the content in the "documents" folder, enabling more contextually relevant responses during queries.

## User Interaction

Finally, the example enters a loop to interact with the user. It prompts the user for a query, performs searches in both the exact and fuzzy databases, and prints the results.

```rust
    loop {
        print!("Query: ");
        std::io::stdout().flush().unwrap();
        let mut user_question = String::new();
        std::io::stdin().read_line(&mut user_question).unwrap();

        println!(
            "vector: {:?}",
            database
                .search(&user_question, 5)
                .await
                .iter()
                .collect::<Vec<_>>()
        );
        println!(
            "fuzzy: {:?}",
            fuzzy
                .search(&user_question, 5)
                .await
                .iter()
                .collect::<Vec<_>>()
        );
    }
}
```

This loop allows continuous interaction with the Kalosm language model, showcasing the real-time querying capabilities of the file system context.

## Conclusion

The file system context in Kalosm provides a seamless way to integrate document management into your Rust applications. By utilizing resource-augmented generation, you enhance the language model's contextual understanding, enabling more accurate and relevant responses. Feel free to customize the example to suit your specific use cases and explore the vast capabilities of Kalosm.