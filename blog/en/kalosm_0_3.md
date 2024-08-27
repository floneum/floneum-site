# Kalosm 0.3: Derive Parsers, Streaming Transcription, Improved Performance, and More!

Kalosm 0.3 makes it significantly easier to use structured generation, improves transcription, and makes it possible to track model download progress. It also includes performance improvements for text generation and transcription models along with parser improvements

## LLM Performance Improvements

The new version of Kalosm includes significant performance improvements for llama, mistral, and phi models. Performance should be between 2-4x as fast depending on your use case. Here is llama 8b chat running on a M2 Macbook Pro:

```inject-dioxus
video {
    width: "50%",
    muted: true,
    autoplay: true,
    loop: true,
    src: "https://github.com/user-attachments/assets/b7d4a51a-11ad-40b7-af14-e6e7b109f7d7"
}
```


## Sampler Aware Structured Generation

The simplest form of structured generation runs a parser 128,000 times, once for each possible new token. This generally isn't an issue for simple grammars like REGEX because each new token is only a lookup away, but it can be slow for more complex parsers. Kalosm 0.3 introduces a new optimization called sampler aware constrained generation: Instead of parsing every token, we can use the sampler to skip parsing unlikely tokens.


There are generally a few steps between the token probabilities after constraints and the token that gets chosen. Each of those steps that modify the probabilities of the tokens is called a sampler. The top-k sampler is one of the most common samplers. It only samples the top k most likely tokens. Here is what that could look like if we only keep the top 2 tokens (where k=2):

![Top-k Sampling](./public/assets/top_k_sampling.png)

> Generally, k is larger, but much smaller than the number of tokens in the model. The default in kalosm is 64.

Instead of parsing every token, we can skip parsing tokens once we find the most probable k tokens. Since the LLM knows about the constraints, the valid tokens tend to have a very high probability so we can skip parsing the majority of the 128,000 tokens.

![Top-K Accelerated Structured Generation](./public/assets/top_k_accelerated_structured_generation.png)

With the new version of Kalosm, loose constraints add almost no overhead to the generation process. Combined with static section acceleration, structured generation is often faster than unconstrained generation. Here is phi-3.5-mini running on a M2 Macbook Pro:

```inject-dioxus
video {
    width: "50%",
    muted: true,
    autoplay: true,
    loop: true,
    src: "https://github.com/user-attachments/assets/cf079495-58db-4397-929c-dba97d877532"
}
```


You can read more about how structured generation works in kalosm in [our last blog post](https://floneum.com/blog/structured_generation_visualized).


## Structured Generation Improvements

Structured generation is also much easier to use in 0.3. Many structured generation tasks can use JSON. If you just need a JSON parser, kalosm 0.3 lets you derive the parser from your data:
```rust
use kalosm::language::*;

/// A fictional character
#[derive(Parse, Schema, Clone, Debug)]
struct Character {
    /// The name of the character
    #[parse(pattern = "[A-Z][a-z]{2,10} [A-Z][a-z]{2,10}")]
    name: String,
    /// The age of the character
    #[parse(range = 1..=100)]
    age: u8,
    /// A description of the character
    #[parse(pattern = "[A-Za-z ]{40,200}")]
    description: String,
}
```

Then you can build a task that generates the structure you defined:

```rust
#[tokio::main]
async fn main() {
    // First create a model. Chat models tend to work best with structured generation
    let model = Llama::new_chat().await.unwrap();
    // Then create a task with the parser as constraints
    let task = Task::builder_for::<Character>("You generate realistic JSON placeholders for characters")
        .build();
    // Finally, run the task
    let mut stream = task.run("Create a random character", &model);
    stream.to_std_out().await.unwrap();
    let character = stream.await.unwrap();
    println!("{character:?}");
}
```

Along with the parser, you can also derive a JSON schema that matches the parser which is useful for function calling models.

## Streaming Voice Transcription

Kalosm 0.3 adds support for transcribing an audio stream in chunks based on voice activity. You can now read the audio stream directly from the microphone and transcribe it as voices are detected:

```rust
// Create a new whisper model.
let model = Whisper::new().await.unwrap();

// Stream audio from the microphone
let mic = MicInput::default();
let stream = mic.stream().unwrap();

// Transcribe the audio into text in chunks based on voice activity.
let mut text_stream = stream.transcribe(model);

// Finally, print the text to the console
text_stream.to_std_out().await.unwrap();
``` 

## Model Progress

Loading models is now async with a callback for loading progress:

```rust
let model = Bert::builder()
    // build with loading handler lets you track the progress of the model loading
    .build_with_loading_handler(|loading| match loading {
        ModelLoadingProgress::Downloading {
            source,
            start_time,
            progress,
        } => {
            let elapsed = start_time.elapsed();
            println!("Downloading model from {source}...{progress}% (elapsed {elapsed:?})");
        }
        ModelLoadingProgress::Loading { progress } => {
            println!("Loading model into memory...{progress}%");
        }
    })
    .await
    .unwrap();
```

Whisper transcriptions and Wuerstchen image generations are also async with progress info thanks to [@newfla](https://github.com/newfla):

```rust
// Create a new whisper model
let model = WhisperBuilder::default()
    .with_source(WhisperSource::QuantizedDistilLargeV3)
    .build()
    .await.unwrap();

let mic = MicInput::default(); 
let audio = mic.stream().unwrap();

// Transcribe the source audio into text
let mut text = audio.transcribe(model);

// As the model transcribes the audio, print the text to the console
while let Some(chunk) = text.next().await {
    let text = chunk.as_ref();
    println!("{text}");
    println!(
        "estimated time left to decode chunk: {}s",
        chunk.remaining_time().as_secs()
    );
}
```

## Documentation improvements

The inline documentation has been significantly improved in 0.3. Common items now include inline guides to help you get started like the [language page](https://docs.rs/kalosm/0.3.2/kalosm/language/index.html) and concept explanations like [embeddings](https://docs.rs/kalosm/0.3.2/kalosm/language/struct.Embedding.html)

```inject-dioxus
div {
    class: "flex flex-row items-center justify-center",
    img {
        src: asset!("./public/assets/docs_1.png"),
        width: "25%",
    }
    img {
        src: asset!("./public/assets/docs_2.png"),
        width: "25%",
    }
    img {
        src: asset!("./public/assets/docs_3.png"),
        width: "25%",
    }
    img {
        src: asset!("./public/assets/docs_4.png"),
        width: "25%",
    }
}
```

## New models!

Along with the new release, kalosm supports a few new models:

- Quantized whisper models are now supported with presets for distilled versions of whisper to run even faster
- The Phi-3 series of models is supported by `kalosm-llama`. The Phi series performs above its weight for structured JSON generation tasks

## New Contributors
* @KerfuffleV2 made their first contribution in [#77](https://github.com/floneum/floneum/pull/77)
* @haoxins made their first contribution in [#86](https://github.com/floneum/floneum/pull/86)
* @Yevgnen made their first contribution in [#93](https://github.com/floneum/floneum/pull/93)
* @dependabot made their first contribution in [#156](https://github.com/floneum/floneum/pull/156)
* @newfla made their first contribution in [#165](https://github.com/floneum/floneum/pull/165)
* @LafCorentin made their first contribution in [#166](https://github.com/floneum/floneum/pull/166)
* @eltociear made their first contribution in [#249](https://github.com/floneum/floneum/pull/249)
