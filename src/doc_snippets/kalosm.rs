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
    async fn main() {
        let mut model = Llama::new_chat();

        // New code
        let mut chat = Chat::builder(&mut model)
            .with_system_prompt("The assistant will act like a pirate")
            .build();

        loop {
            chat.add_message(prompt_input("\n> ").unwrap())
                .await
                .unwrap()
                .to_std_out()
                .await
                .unwrap();
        }
    }
    // ANCHOR_END: second
}
