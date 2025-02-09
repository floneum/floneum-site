# Text Generation with Kalosm in Rust

## Introduction

Kalosm supports two different model types: complete and streaming. Completion models are trained to complete free-form text. Chat models are trained with a chat format and can be used to generate responses to user prompts. 

## Building a Local Model

You can use `Llama::builder()` to create a local completion or chat model. You can set the source of the model with `with_source` to a local or remote file:

```rust
{{#include src/doc_snippets/llms.rs:create_llama_model}}
```

> Bonus: Download progress
> 
> If you need to update progress while you are downloading the model, you can use the bert builder with the `build_with_loading_handler` method.
> 
> ```rust
> {{#include src/doc_snippets/llms.rs:create_model_with_loading_handler}}
> ```


## Building a Remote Model

For remote chat models, you can use `OpenAICompatibleChatModel::builder()` to create a connection to a remote model. You can set a specific model with one of the presets, or set a custom model id with the `with_model` method:

```rust
{{#include src/doc_snippets/llms.rs:create_remote_model}}
```

## Text Completion

Once you have a text completion model, you can call the model like a function with the text you need to complete. Before you await the response, you can modify the builder with additional settings like the maximum length of the generated text. Once you are done, you can await the response to get the full generated text:

```rust
{{#include src/doc_snippets/llms.rs:text}}
```

Instead of awaiting the full result, you can treat the response builder like a stream and read each token as it is generated:

```rust
{{#include src/doc_snippets/llms.rs:streaming_text}}
```

## Chat Models

If you have a chat model like `OpenAICompatibleChatModel` or `Llama` with a chat source, you can use the `chat` method to start a chat session with the model. You can call the chat session with a new message to create a response builder. You can modify the builder with settings and then await the response or stream the response:

```rust
{{#include src/doc_snippets/llms.rs:chat}}
```

## Conclusion

This chapter demonstrated both completion and chat models. Experiment with different prompts, configurations, and models to customize the generated text to your needs. If you need more control over the text generation process, check out the next chapter on [structured generation](./structured_generation/index.md)
