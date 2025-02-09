#[tokio::main]
async fn main() {
    // ANCHOR: create_llama_model
    use kalosm::language::*;
    use std::path::PathBuf;

    // Create a builder for a chat model
    let phi = Llama::builder()
        // Set the source of the model
        .with_source(LlamaSource::phi_3_5_mini_4k_instruct())
        // Build the model. This will fetch the model from the source if it is not cached.
        .build()
        .await
        .unwrap();

    // You can also create a model from a local file
    let model = Llama::builder()
        // To use a custom model, you can create a new LlamaSource
        .with_source(LlamaSource::new(
            // Llama source takes a gguf file to load the model, tokenizer, and chat template from
            FileSource::Local(PathBuf::from("path/to/model.gguf")),
        ))
        .build()
        .await
        .unwrap();
    // ANCHOR_END: create_llama_model

    // ANCHOR: text
    let text = model("The capital of France is")
        // Set any options you need
        .with_sampler(GenerationParameters::default().with_max_length(1000))
        // Once you are done, call `await` to get the generated text
        .await
        .unwrap();
    println!("{}", text);
    // ANCHOR_END: text

    // ANCHOR: streaming_text
    let mut text_stream = model("The capital of France is")
        // Set any options you need
        .with_sampler(GenerationParameters::default().with_max_length(1000));

    // Pipe the generated text to stdout or read individual tokens with the `next` method from the stream ext trait
    text_stream.to_std_out().await.unwrap();
    // ANCHOR_END: streaming_text

    // ANCHOR: chat
    // Create a chat session
    let mut chat = phi.chat();

    loop {
        let message = prompt_input("\n> ").unwrap();
        // Add a message to the chat session
        let mut response = chat(&message);

        // Stream the response to stdout
        response.to_std_out().await.unwrap();
    }
    // ANCHOR_END: chat
}

fn create_remote_model() {
    use kalosm::language::*;

    // ANCHOR: create_remote_model
    let llm = OpenAICompatibleChatModel::builder()
        .with_gpt_4o_mini()
        .build();
    // ANCHOR_END: create_remote_model
}

async fn create_model_with_loading_handler() {
    use kalosm::language::*;

    // ANCHOR: create_model_with_loading_handler
    // You can also create a model from a local file
    let model = Llama::builder()
        .with_source(LlamaSource::phi_3_5_mini_4k_instruct())
        // You can call build_with_loading_handler to get progress updates
        // as the model is being downloaded and loaded
        .build_with_loading_handler(|progress| match progress {
            ModelLoadingProgress::Downloading { source, progress } => {
                let progress_percent = (progress.progress * 100) as u32;
                let elapsed = progress.start_time.elapsed().as_secs_f32();
                println!("Downloading file {source} {progress_percent}% ({elapsed}s)");
            }
            ModelLoadingProgress::Loading { progress } => {
                let progress = (progress * 100.0) as u32;
                println!("Loading model {progress}%");
            }
        })
        .await
        .unwrap();
    // ANCHOR_END: create_model_with_loading_handler
}
