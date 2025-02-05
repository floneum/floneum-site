#[tokio::main]
async fn main() {
    // ANCHOR: create_chat_model
    use kalosm::language::*;
    let model = Llama::new_chat().await.unwrap();
    // ANCHOR_END: create_chat_model

    // ANCHOR: create_chat_wrapper
    let mut chat = model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");
    // ANCHOR_END: create_chat_wrapper

    // ANCHOR: conversation_loop
    loop {
        chat(&prompt_input("\n> ").unwrap())
            .to_std_out()
            .await
            .unwrap();
    }
    // ANCHOR_END: conversation_loop
}
