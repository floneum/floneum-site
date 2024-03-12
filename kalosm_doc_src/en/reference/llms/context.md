# Text Embeddings

Embeddings are a numerical representation of the meaning of some data. Text embeddings represent something about the meaning of some text. Embeddings can be used either directly or as input to machine learning models. In this chapter, we will learn how to use Kalosm to create text embeddings and integrate them into a database. We will also learn how to search the embedding database for documents that are similar to a given query.

## Creating an Embedding model

First, we need to create an embedding model. An embedding model is a machine learning model that can be used to create embeddings. Kalosm provides a `Bert` struct that can be used to create an embedding model.

```rust
{{#include src/doc_snippets/embeddings.rs:create_embedding_model}}
```

## Creating Embeddings

Once we have created an embedding model, we can use it to create embeddings. Kalosm provides a `Bert` struct that can be used to create embeddings.

```rust
{{#include src/doc_snippets/embeddings.rs:create_embeddings}}
```

Try different values for the text we are embedding. How does the embedding change?

## Creating an Embedding Database

Now that we know how to create embeddings, we can use them to create an embedding database. An embedding database is a data structure that stores embeddings and allows you to search for documents that are similar to a given query. Kalosm provides a `DocumentTable` struct that can be used to create an embedding database linked to a table in a [Surrealdb](https://surrealdb.com/) database. You can choose a chunk strategy to use when creating the embedding database. A chunk strategy determines how documents are split into chunks before being embedded. In this example, we will use the `Sentence` chunk strategy, which splits documents into sentences before embedding them. The bert embedding model tends to work best with single sentence chunks.

```rust
{{#include src/doc_snippets/embeddings.rs:create_embedding_database}}
```

### Adding Documents to the Embedding Database

Once you have created an embedding database, you can add documents to it with the `extend` method. The `extend` method takes something that can be turned into documents and adds them to the embedding database. In this example, we will add documents from a RSS feed to the embedding database.

```rust
{{#include src/doc_snippets/embeddings.rs:extend_database}}
```

> This example uses rss context, but you can also use [audio](https://github.com/floneum/floneum/blob/main/interfaces/kalosm/examples/live_qa.rs), [filesystem](https://github.com/floneum/floneum/blob/94542cf49923cb4e15e34244336f8844ee2194c4/interfaces/kalosm/examples/fs_context.rs#L31), or [search engine](https://github.com/floneum/floneum/blob/94542cf49923cb4e15e34244336f8844ee2194c4/interfaces/kalosm-language/src/context/search/mod.rs#L16-L31) context
> You can also use a [fuzzy search engine](https://github.com/floneum/floneum/blob/94542cf49923cb4e15e34244336f8844ee2194c4/interfaces/kalosm/examples/fs_context.rs#L19-L20) with the same api if you prefer traditional search

### Searching the Embedding Database

Next, you can use search through the documents you embedded with the `search` method. The `search` method takes a query and returns a list of documents that are similar to the query. The `search` method also takes a `limit` parameter that determines how many documents to return.

```rust
{{#include src/doc_snippets/embeddings.rs:search_database}}
```

## Conclusion

In this chapter, we learned how to use Kalosm to create embeddings and integrate them into a database. We also learned how to search the embedding database for similar documents.
