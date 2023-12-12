#[tokio::main]
async fn main() {
    // ANCHOR: create_chat_model
    let mut model = Llama::new_chat();
    // Create a builder for a chat model
    let mut model = Llama::builder()
        // Set the source of the model
        .with_source(LlamaSource::mistral_7b())
        // Build the model. This will fetch the model from the source if it is not cached.
        .build()
        .unwrap();
    // ANCHOR_END: create_chat_model

    // ANCHOR: create_chat_wrapper
    let mut chat = Chat::builder(&mut model)
      .with_system_prompt("The assistant will act like a pirate")
      .build();
    // ANCHOR_END: create_chat_wrapper

    // ANCHOR: streaming_text
    loop {
        chat.add_message(prompt_input("\n> ").unwrap())
           .await
           .unwrap()
           .to_std_out()
           .await
           .unwrap();
    }
    // ANCHOR_END: streaming_text
}
