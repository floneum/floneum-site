mod first {
    // ANCHOR: first
    use kalosm::language::*;

    #[tokio::main]
    async fn main() {
        let _model = Llama::new_chat();
    }
    // ANCHOR_END: first
}
mod second {
    // ANCHOR: second
    use kalosm::language::*;

    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        let model = Llama::new_chat().await?;

        // New code
        let mut chat = Chat::builder(model)
            .with_system_prompt("The assistant will act like a pirate")
            .build();

        loop {
            let message = prompt_input("\n> ")?;
            chat.add_message(message).to_std_out().await?;
        }
    }
    // ANCHOR_END: second
}
