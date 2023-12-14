#[tokio::main]
async fn main() {
    // ANCHOR: create_model
    use kalosm::language::*;
    // Create a builder for a chat model
    let mut model = Llama::builder()
        // Set the source of the model
        .with_source(LlamaSource::mistral_7b())
        // Build the model. This will fetch the model from the source if it is not cached.
        .build()
        .unwrap();
    // ANCHOR_END: create_model

    // ANCHOR: synchronous_text
    let text = model
        .generate_text("The capital of France is")
        // Set any options you need
        .with_max_length(300)
        // Once you are done, call `await` to get the generated text
        .await
        .unwrap();
    println!("{}", text);
    // ANCHOR_END: synchronous_text

    // ANCHOR: streaming_text
    let text_stream = model
        .stream_text("The capital of France is")
        // Set any options you need
        .with_max_length(1000)
        // Call `await` to get a stream of text tokens
        .await
        .unwrap();

    // Pipe the generated text to stdout or read individual tokens with the `next` method from the stream ext trait
    text_stream.to_std_out().await.unwrap();
    // ANCHOR_END: streaming_text
}
