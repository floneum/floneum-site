# Introducing Kalosm 0.4: A Local-First AI Meta-Framework for Rust

Kalosm is a simple, local first interface for pre-trained language, audio, and image models written in Rust. It is built on top of the [candle](https://github.com/huggingface/candle) library for ML inference. Kalosm makes it easy to run quantized models on your local machine accelerated by cuda or metal. Whether you’re building a chatbot, a transcription tool, or an image processing application, Kalosm 0.4 brings major improvements to help you get started quickly and efficiently.

In this release, we’re excited to announce:

- **A Simplified API** for language models, tasks, and chat sessions
- **Extended support for remote models** via OpenAI and Anthropic adapters with structured generation
- **Single-file model loading** that consolidates model weights, tokenizer, and chat template metadata into one GGUF file
- **Improved whisper transcription** with word-level timestamps using dynamic time warping

---

## Simplified LLM API

One of the biggest improvements in Kalosm 0.4 is the streamlined API for interacting with local large language models. Developers can now configure chat sessions and tasks with a series of builder methods until the first message is added. This flexibility simplifies the code and makes your applications easier to maintain.

For example, here’s how you can set up a chat session with a system prompt:

```rust
use kalosm::language::*;

// Create a new chat-capable Llama model
let model = Llama::new_chat().await?;

// Configure a chat session with a system prompt
let mut chat = model
    .chat()
    .with_system_prompt("The assistant will act like a pirate");

loop {
    // Get user input and stream the model’s response to stdout
    chat(&prompt_input("\n> ")?)
        .to_std_out()
        .await?;
}
```

Similarly, the task API now supports automatic constraint inference. By deriving parsing and schema traits on your data types, you can easily generate concrete types from your language model:

```rust
use kalosm::language::*;

/// A fictional account holder
#[derive(Schema, Parse, Clone, Debug)]
struct Account {
    /// A brief summary of the account holder
    #[parse(pattern = r"[a-zA-Z,.?!\d ]{1,80}")]
    summary: String,
    /// The account holder’s name (full name or pseudonym)
    #[parse(pattern = "[a-zA-Z ]{1,20}")]
    name: String,
    /// The account holder’s age
    #[parse(range = 1..=100)]
    age: u8,
}

// Create a small reasoning-focused model
let llm = Llama::phi_3().await?;

// Create a task for generating accounts, with automatic type-based constraint inference
let create_account = llm
    .task("You generate accounts based on a description of the account holder")
    .typed();

// Generate an account from a natural language prompt
let account: Account = create_account(
    "Candice is the CEO of a fortune 500 company. She is 30 years old."
).await?;

println!("Generated Account: {account:?}");
```

---

## OpenAI and Anthropic Support with Structured Generation

Kalosm 0.4 extends its support for remote models by integrating with the OpenAI and Anthropic chat APIs. Now you can use Kalosm’s structured generation even when working with remote chat models. Unfortunately Anthropic currently doesn't support structured generation, so support for tasks with Anthropic models is currently limited.

To work with OpenAI models, enable the `openai` feature and set your API key via the `OPENAI_API_KEY` environment variable. Then create a model adapter:

```rust
// Create a chat model adapter for OpenAI-compatible models
let llm = OpenAICompatibleChatModel::builder()
    .with_gpt_4o_mini()
    .build();

// Use the adapter just like any other chat model
let generate_character = llm
    .task("You generate accounts based on a description of the account holder")
    .typed();

let account: Account = generate_character(
    "Candice is the CEO of a fortune 500 company. She is 30 years old."
).await?;
println!("Generated Account: {account:?}");
```

For Anthropic models, enable the `anthropic` feature and set your `ANTHROPIC_API_KEY`:

```rust
// Create a chat model adapter for Anthropic-compatible models
let llm = AnthropicCompatibleChatModel::builder()
    .with_claude_3_5_haiku()
    .build();

// Start a chat session with a custom system prompt
let mut chat = llm
    .chat()
    .with_system_prompt("The assistant will act like a pirate");

loop {
    chat(&prompt_input("\n> ")?)
        .to_std_out()
        .await?;
}
```

These integrations allow you to combine local inference with remote model capabilities and structured generation features, making it easier to build hybrid AI applications where local models may not be enough.

---

## Single-File LLM Loading

Previously, loading a custom model required managing separate files for the tokenizer, chat template, and model weights. With version 0.4, Kalosm supports loading all model metadata from a single GGUF file. This improvement simplifies model distribution and deployment.

To load a custom model, you can now just point to a local or huggingface GGUF file:

```rust
let model = Llama::builder()
    // Specify a custom model source using a GGUF file
    .with_source(LlamaSource::new(
        FileSource::HuggingFace {
            model_id: "QuantFactory/SmolLM-1.7B-Instruct-GGUF".to_string(),
            revision: "main".to_string(),
            file: "SmolLM-1.7B-Instruct.Q4_K_M.gguf".to_string(),
        },
    ))
    .build()
    .await?;

let mut chat = model
    .chat()
    .with_system_prompt("The assistant will act like a pirate");

loop {
    chat(&prompt_input("\n> ")?)
        .to_std_out()
        .await?;
}
```

This unified approach to model loading makes it much easier to switch between different models and update to new models over time.

---

## Timestamped Transcription with Whisper

Kalosm 0.4 also brings improvements to audio transcription. The new Whisper API supports word-level timestamps, which can be critical for applications that require precise alignment between audio and text.

Below is an example that demonstrates how to transcribe audio and print each segment along with its corresponding timestamps:

```rust
// Build a new Whisper model using a quantized variant
let model = WhisperBuilder::default()
    .with_source(WhisperSource::QuantizedLargeV3Turbo)
    .build()
    .await?;

// Open and decode an audio file
let file = BufReader::new(File::open("./samples_jfk.wav")?);
let audio = Decoder::new(file)?;

// Transcribe the audio with timestamping enabled
let mut text = model.transcribe(audio).timestamped();

// Print each transcribed segment with its start and end times
while let Some(segment) = text.next().await {
    for chunk in segment.chunks() {
        let timestamp = chunk.timestamp().unwrap();
        println!("{:0.2}..{:0.2}: {}", timestamp.start, timestamp.end, chunk);
    }
}
```

This level of granularity in transcription is particularly useful for applications like video captioning, meeting transcription, or any context where knowing the exact timing of spoken words matters.

---

## Community and Support

Kalosm is built by a passionate community dedicated to making local AI inference accessible and efficient. For help, discussion, or to share feedback, join the [Kalosm Discord community](https://discord.gg/dQdmhuB8q5). Whether you’re troubleshooting, looking to contribute, or simply curious about the latest updates, our Discord is the best place to connect with fellow developers and the Kalosm team.

---

## Conclusion

Kalosm 0.4 brings a range of enhancements—from simplified APIs and unified model loading to extended remote support and precise transcription capabilities—that we hope will make your development process smoother and more enjoyable. We're truly excited to see the creative applications and projects you build with these tools. 

Happy coding!
