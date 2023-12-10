# Text Embeddings and Embedding Databases

Embeddings are a way to represent words or phrases as vectors of numbers. These vectors can be used as input to machine learning models. In Kalosm, you can use the `DocumentDatabase` struct to create an embedding database from a collection of documents.

## Creating Embeddings

To create embeddings using Kalosm, you can follow these steps:

1. Create a `Bert` object by calling `Bert::builder().build().unwrap()`.
2. Create a `Document` object by calling `Document::new(text)`, where `text` is the text you want to create embeddings for.
3. Call the `Bert::embed` method on the `Bert` object and pass in the `Document` object.

Here is an example of how to create embeddings using Kalosm:

```rust
use kalosm::language::*;

fn main() {
    let bert = Bert::builder().build().unwrap();
    let document = Document::new("Hello, world!");
    let embeddings = bert.embed(&document).unwrap();
    println!("{:?}", embeddings);
}
```

## Creating an Embedding Database

To create an embedding database using Kalosm, you can follow these steps:

1. Create a `DocumentFolder` object that points to the directory containing your documents.
2. Create a `DocumentDatabase` object by passing a pre-trained model and a chunk strategy to the constructor.
3. Call the `extend` method on the `DocumentDatabase` object and pass in the `DocumentFolder` object.

Here is an example of how to create an embedding database using Kalosm:

```rust
use kalosm::language::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let documents = DocumentFolder::try_from(PathBuf::from("./documents")).unwrap();

    let mut database = DocumentDatabase::new(
        Bert::builder().build().unwrap(),
        ChunkStrategy::Sentence {
            sentence_count: 1,
            overlap: 0,
        },
    );
    database.extend(documents.clone()).await.unwrap();
}
```

### Searching the Embedding Database

Once you have created an embedding database, you can use Kalosm's `FuzzySearchIndex` to search the database using fuzzy search.

To search the embedding database using Kalosm's `FuzzySearchIndex`, you can follow these steps:

1. Create a `FuzzySearchIndex` object by calling `FuzzySearchIndex::default()`.
2. Call the `extend` method on the `FuzzySearchIndex` object and pass in the `DocumentFolder` object.
3. Call the `search` method on the `FuzzySearchIndex` object and pass in the query string and the number of results you want.

Here is an example of how to search the embedding database using Kalosm's `FuzzySearchIndex`:

```rust
use kalosm::language::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let documents = DocumentFolder::try_from(PathBuf::from("./documents")).unwrap();

    let mut fuzzy = FuzzySearchIndex::default();
    fuzzy.extend(documents).await.unwrap();

    let results = fuzzy.search("query", 5).await;
    println!("{:?}", results);
}
```

## Conclusion

In this chapter, we learned how to use Kalosm to create embeddings and integrate them into a database. We also learned how to search the embedding database using fuzzy search. Embeddings are a powerful tool for machine learning, and Kalosm makes it easy to work with them in Rust. If you have any questions or feedback, please let us know!

I hope this version is better. Let me know if you have any further questions.
