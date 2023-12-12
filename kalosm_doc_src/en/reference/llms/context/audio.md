# Chapter: Audio Context in Kalosm

## Introduction

In this chapter, we'll explore the integration of audio context within Kalosm, showcasing how you can build a conversational application that leverages both audio transcription and document-based context. The example combines the power of the Kalosm language model, audio processing with Whisper, and document search with FuzzySearchIndex.

## Setting Up the Project

Ensure that you have the required dependencies in your `Cargo.toml`:

```toml
[dependencies]
kalosm = "0.1"
kalosm_language = "0.1"
kalosm_sound = "0.1"
tokio = { version = "1", features = ["full"] }
futures-util = "0.3"
```

This example utilizes the `Whisper` audio processing library and `futures-util` for asynchronous stream handling. Make sure to add these dependencies to your project.

## Initializing Audio and Document Context

```rust
use futures_util::StreamExt;
use kalosm::*;
use kalosm_language::*;
use kalosm_sound::*;
use std::sync::Arc;
use tokio::{
    sync::RwLock,
    time::{Duration, Instant},
};
```

Initialize the Whisper model and the FuzzySearchIndex for document-based context:

```rust
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let model = WhisperBuilder::default()
        .with_source(WhisperSource::MediumEn)
        .build()?;
    
    let document_engine = Arc::new(RwLock::new(FuzzySearchIndex::default()));
    // ...
}
```

Here, we create a `Whisper` model for audio transcription and a `FuzzySearchIndex` for document-based context. The `document_engine` is wrapped in an `Arc<RwLock<>>` for thread-safety.

## Recording Audio and Augmenting Context

The example utilizes a separate thread to continuously record audio and augment the document context:

```rust
    let document_engine = document_engine.clone();
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let recording_time = Duration::from_secs(30);
                loop {
                    let input = kalosm_sound::MicInput::default()
                        .record_until(Instant::now() + recording_time)
                        .await
                        .unwrap();

                    if let Ok(mut transcribed) = model.transcribe(input) {
                        while let Some(transcribed) = transcribed.next().await {
                            if transcribed.probability_of_no_speech() < 0.90 {
                                let text = transcribed.text();
                                document_engine.write().await.add(text).await.unwrap();
                            }
                        }
                    }
                }
            })
    });
```

This snippet starts a thread that records audio for 30 seconds, transcribes it using the Whisper model, and adds the relevant text to the document context.

## Conversational Interaction

The main loop of the example handles user input, retrieves relevant context from the document engine, and prompts the user with a context-aware question:

```rust
    let mut model = Llama::new_chat();
    let mut chat = Chat::builder(&mut model)
        .with_system_prompt("The assistant helps answer questions based on the context given by the user. The model knows that the information the user gives it is always true.")
        .build();

    loop {
        let user_question = prompt_input("\n> ").unwrap();

        let mut engine = document_engine.write().await;
        let context = {
            let context = engine.search(&user_question, 5).await;
            let context = context
                .iter()
                .take(5)
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            context.join("\n")
        };

        let prompt = format!(
            "Here is the relevant context:\n{context}\nGiven that context, answer the following question:\n{user_question}"
        );

        println!("{}", prompt);

        let output_stream = chat.add_message(prompt).await.unwrap();
        print!("Bot: ");
        output_stream.to_std_out().await.unwrap();
    }
}
```

This loop prompts the user for input, searches the document context for relevant information, and generates a context-aware question using the Kalosm language model. The assistant responds based on the given context and user input.

## Conclusion

By combining audio context with document-based context, you can create more dynamic and context-aware conversational applications using Kalosm. This example demonstrates the seamless integration of audio transcription and document search, allowing your application to provide more informed and relevant responses. Customize and extend this example to suit your specific use cases and explore the full potential of Kalosm in audio-driven conversational interfaces.