# Resource Augmented Generation in Kalosm

## Introduction

Resource augmented generation in Kalosm is a powerful approach that combines natural language generation with real-time information from audio data. This guide will walk you through the process of implementing resource augmented generation using the Kalosm library.

> Before you begin, make sure you have the Kalosm library installed and set up in your Rust project. You can refer to the [Introduction](../index.md) for instructions on how to install Kalosm.

## Creating a Transcription Model

First we need to create a transcription model. Kalosm provides a `Whisper` struct that serves as a transcription model. You can initialize it as follows:

```rust
{{#include src/doc_snippets/live_qa.rs:create_whisper}}
```

This sets up a transcription model using the Whisper source for English (`WhisperSource::MediumEn`). Adjust the source according to your language preferences.

## Creating a Context Database

The next step is to create a context database. Kalosm provides `DocumentTable` struct that indexes a [Surrealdb](https://surrealdb.com/) database table with a vector database which can serve as our context database. We need to wrap the database in a `Arc` so that it can be shared across threads:

```rust
{{#include src/doc_snippets/live_qa.rs:create_context_db}}
```

## Recording Audio

Next, we need to record audio data. Kalosm provides a `MicInput` struct that can be used to record audio data.

```rust
{{#include src/doc_snippets/live_qa.rs:record_audio}}
```

This code records audio until a certain point in time, providing a continuous stream of audio data.

## Transcribing Audio

Once you have recorded audio data, you can transcribe it into text using the transcription model. The `transcribe` method returns a stream of text snippets along with the confidence of the transcription. We can add the text snippets to the context database to create a real-time context.

```rust
{{#include src/doc_snippets/live_qa.rs:transcribe_audio}}
```

## Create Chat Model

Next, we need to create a chat model. We can use the default chat model provided by Kalosm along with the `Chat` interface.

```rust
{{#include src/doc_snippets/live_qa.rs:create_chat}}
```

## Implementing Resource Augmented Generation

Finally, we can implement the main chat loop that asks the user for input, searches the context database for relevant information, and generates a response using the chat model.

```rust
{{#include src/doc_snippets/live_qa.rs:rag}}
```

## Conclusion

Resource augmented generation in Kalosm makes it possible for language models to generate responses based on up-to-date information from the real world. This guide has shown you how to implement resource augmented generation using the Kalosm library. For more information, the reference documentation documents more details about: [Whisper](../reference/transcription.md), [DocumentTable](../reference/llms/context.md), and [Chat](../reference/llms/chat.md).
