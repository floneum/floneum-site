# Text Generation with Kalosm in Rust

## Introduction

Kalosm, provides both synchronous and streaming approaches for generating text. This chapter explores both methods, offering flexibility based on your specific requirements.

## Setting up the Kalosm Model

Initializing the Kalosm model is the first step in both synchronous and streaming text generation. In this example, we use the model Mistral 7b, but Kalosm also supports other Llama models, Phi, and remote models with the same API.

```rust
{{#include src/doc_snippets/llms.rs:create_model}}
```

## Synchronous Text Generation

For synchronous text generation, use the `generate_text` method. Specify the prompt and any additional configurations, such as the maximum length of the generated text:

```rust
{{#include src/doc_snippets/llms.rs:synchronous_text}}
```

## Streaming Text Generation

Streaming text generation allows efficient handling of large amounts of generated text. Use the `stream_text` method to initiate the generation process and iterate over the generated tokens:

```rust
{{#include src/doc_snippets/llms.rs:streaming_text}}
```

Just like the synchronous method, you can specify additional configurations, such as the maximum length of the generated text before awaiting the stream to start generating text.

## Conclusion

This chapter demonstrated both synchronous and streaming approaches to text generation using Kalosm in Rust. The synchronous method is straightforward and suitable for smaller text generation tasks, while the streaming method efficiently handles larger amounts of generated text. Experiment with different prompts, configurations, and data sources to customize the generated text to your needs. Explore the Kalosm documentation for additional features and options available in the framework. Happy text generation with Kalosm!

If you need even more control over the text generation process, check out the next chapter on the Kalosm API: [structured generation](./structured_generation/index.md)
