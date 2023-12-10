# Text Generation with Kalosm in Rust - Synchronous and Streaming

## Introduction

Kalosm, provides both synchronous and streaming approaches for generating text. This chapter explores both methods, offering flexibility based on your specific requirements.

## Setting up the Kalosm Model

Initializing the Kalosm model is the first step in both synchronous and streaming text generation. In this example, we use the Mistral model Zephyr 7B Beta, but Kalosm also supports Phi, and remote models with the same API.

```rust
use kalosm::language::*;

fn main() {
    let mut model = Llama::builder()
        .with_source(LlamaSource::zephyr_7b_beta())
        .build()
        .unwrap();
}
```

## Synchronous Text Generation

For synchronous text generation, use the `generate_text` method. Specify the prompt and any additional configurations, such as the maximum length of the generated text:

```rust
use kalosm::language::*;

fn main() {
    let mut model = Llama::builder()
        .with_source(LlamaSource::zephyr_7b_beta())
        .build()
        .unwrap();
    
    // New code
    let text = model.generate_text(prompt).with_max_length(300).unwrap();
    println!("{}", text);
}
```

## Streaming Text Generation

Streaming text generation allows efficient handling of large amounts of generated text. Use the `stream_text` method to initiate the generation process and iterate over the generated tokens:

```rust
use kalosm::language::*;
use std::io::Write;

fn main() {
    let mut model = Llama::builder()
        .with_source(LlamaSource::zephyr_7b_beta())
        .build()
        .unwrap();

    // New code
    let prompt = "The capital of France is";
    print!("{prompt}");
    let mut text_stream = model
        .stream_text(prompt)
        .with_max_length(1000)
        .await
        .unwrap();

    while let Some(token) = text_stream.next().await {
        print!("{token}");
        std::io::stdout().flush().unwrap();
    }
}
```

Just like the synchronous method, you can specify additional configurations, such as the maximum length of the generated text before awaiting the stream to start generating text.

## Conclusion

This chapter demonstrated both synchronous and streaming approaches to text generation using Kalosm in Rust. The synchronous method is straightforward and suitable for smaller text generation tasks, while the streaming method efficiently handles larger amounts of generated text. Experiment with different prompts, configurations, and data sources to customize the generated text to your needs. Explore the Kalosm documentation for additional features and options available in the framework. Happy text generation with Kalosm!

If you need even more control over the text generation process, check out the next chapter on the Kalosm API: [structured generation](./structured_generation.md)
