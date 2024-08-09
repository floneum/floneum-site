#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // ANCHOR: create_whisper
    use kalosm::language::*;
    use kalosm::sound::*;
    use kalosm::*;
    use std::sync::Arc;
    use tokio::time::{Duration, Instant};
    let model = WhisperBuilder::default()
        .with_source(WhisperSource::MediumEn)
        .build()
        .await?;
    // ANCHOR_END: create_whisper

    // ANCHOR: create_context_db
    // Create database connection
    let db = surrealdb::Surreal::new::<surrealdb::engine::local::RocksDb>("./db/temp.db").await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a new document database table
    let document_table = Arc::new(
        db.document_table_builder("documents")
            // Store the embedding database at ./db/embeddings.db
            .at("./db/embeddings.db")
            .build()
            .await?,
    );

    // ANCHOR_END: create_context_db
    // ANCHOR: record_audio
    {
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let recording_time = Duration::from_secs(30);
                    loop {
                        let _input = MicInput::default()
                            .record_until(Instant::now() + recording_time)
                            .await
                            .unwrap();
                    }
                })
        });
    }
    // ANCHOR_END: record_audio

    // ANCHOR: transcribe_audio
    {
        let document_table = document_table.clone();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let recording_time = Duration::from_secs(30);
                    loop {
                        let input = MicInput::default()
                            .record_until(Instant::now() + recording_time)
                            .await
                            .unwrap();

                        if let Ok(mut transcribed) = model.transcribe(input) {
                            while let Some(transcribed) = transcribed.next().await {
                                if transcribed.probability_of_no_speech() < 0.90 {
                                    let document =
                                        transcribed.text().into_document().await.unwrap();
                                    document_table.insert(document).await.unwrap();
                                }
                            }
                        }
                    }
                })
        });
    }
    // ANCHOR_END: transcribe_audio

    // ANCHOR: create_chat
    let model = Llama::new_chat().await?;
    let mut chat = Chat::builder(model).with_system_prompt("The assistant help answer questions based on the context given by the user. The model knows that the information the user gives it is always true.").build();
    // ANCHOR_END: create_chat

    // ANCHOR: rag
    loop {
        // Ask the user for a question
        let user_question = prompt_input("\n> ")?;

        // Search for relevant context in the document engine
        let context = document_table
            .select_nearest(&user_question, 5)
            .await?
            .into_iter()
            .map(|document| {
                format!(
                    "Title: {}\nBody: {}\n",
                    document.record.title(),
                    document.record.body()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Format a prompt with the question and context
        let prompt = format!(
            "Here is the relevant context:\n{context}\nGiven that context, answer the following question:\n{user_question}"
        );

        // Display the prompt to the user for debugging purposes
        println!("{}", prompt);

        // And finally, respond to the user
        let mut output_stream = chat.add_message(prompt);
        print!("Bot: ");
        output_stream.to_std_out().await?;
    }
    // ANCHOR_END: rag
}
