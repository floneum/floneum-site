# Audio Transcription

Kalosm provides helpers that allow you to easily transcribe audio data from either your microphone or a file. In this chapter, we will learn how to use Kalosm to perform real time audio transcription.

## Creating a Transcription Model

First, we need to create a transcription model. A transcription model is a machine learning model that can be used to transcribe audio data. Kalosm provides a `Whisper` struct that can be used to create a transcription model.

```rust
{{#include src/doc_snippets/transcription.rs:create_model}}
```

> Bonus: Download progress
> 
> If you need to update progress while you are downloading the model, you can use the whisper builder with the `build_with_loading_handler` method.
> 
> ```rust
> {{#include src/doc_snippets/transcription.rs:create_model_with_loading_handler}}
> ```

## Recording Audio

Next, we need to record some audio from our environment. We can use the `MicInput` struct with the `stream` method to stream audio from our microphone:

```rust
{{#include src/doc_snippets/transcription.rs:record_audio}}
```

## Transcribing Audio

Finally, we can use the `transcribe` method to transcribe the audio data that we recorded into text. The transcribe method takes some audio data and returns a stream of snippets of text along with the confidence of the transcription.

```rust
{{#include src/doc_snippets/transcription.rs:transcribe}}
```

## Conclusion

In this chapter, we learned how to use Kalosm to transcribe audio data from our microphone. Audio data can be a powerful source of real time information. You can combine audio data with [retrieval-augmented generation](../guides/retrieval_augmented_generation.md) to create a [chat bot that understands it's surroundings](https://github.com/floneum/floneum/blob/main/interfaces/kalosm/examples/live_qa.rs).
