mod first {
    // ANCHOR: first
    use kalosm::language::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let model = Llama::new_chat().await?;
    }
    // ANCHOR_END: first
}
mod second {
    // ANCHOR: second
    use kalosm::language::*;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let model = Llama::new_chat().await?;

        // New code
        let mut chat = Chat::builder(model)
            .with_system_prompt("The assistant will act like a pirate")
            .build();

        loop {
            chat.add_message(prompt_input("\n> ")?).to_std_out().await?;
        }
    }
    // ANCHOR_END: second
}
