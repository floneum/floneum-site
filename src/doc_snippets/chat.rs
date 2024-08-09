#[tokio::main]
async fn main() {
    // ANCHOR: create_chat_model
    use kalosm::language::*;
    let model = Llama::new_chat().await.unwrap();
    // ANCHOR_END: create_chat_model

    // ANCHOR: create_chat_wrapper
    let mut chat = Chat::builder(model)
        .with_system_prompt("The assistant will act like a pirate")
        .build();
    // ANCHOR_END: create_chat_wrapper

    // ANCHOR: streaming_text
    loop {
        chat.add_message(prompt_input("\n> ").unwrap())
            .to_std_out()
            .await
            .unwrap();
    }
    // ANCHOR_END: streaming_text
}
