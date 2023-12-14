#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // ANCHOR: create_whisper
    use futures_util::StreamExt;
    use kalosm::*;
    use kalosm_language::*;
    use kalosm_sound::*;
    use std::sync::Arc;
    use tokio::{
        sync::RwLock,
        time::{Duration, Instant},
    };
    let model = WhisperBuilder::default()
        .with_source(WhisperSource::MediumEn)
        .build()?;
    // ANCHOR_END: create_whisper

    // ANCHOR: create_context_db
    let document_engine = Arc::new(RwLock::new(FuzzySearchIndex::default()));
    // ANCHOR_END: create_context_db
    {
        let model = WhisperBuilder::default()
            .with_source(WhisperSource::MediumEn)
            .build()?;
        // ANCHOR: record_audio
        {
            let document_engine = document_engine.clone();
            std::thread::spawn(move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async move {
                        let recording_time = Duration::from_secs(30);
                        loop {
                            let input = kalosm_sound::MicInput::default()
                                .record_until(Instant::now() + recording_time)
                                .await
                                .unwrap();
                        }
                    })
            });
        }
        // ANCHOR_END: record_audio
    }

    {
        let model = WhisperBuilder::default()
            .with_source(WhisperSource::MediumEn)
            .build()?;
        // ANCHOR: transcribe_audio
        {
            let document_engine = document_engine.clone();
            std::thread::spawn(move || {
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async move {
                        let recording_time = Duration::from_secs(30);
                        loop {
                            let input = kalosm_sound::MicInput::default()
                                .record_until(Instant::now() + recording_time)
                                .await
                                .unwrap();

                            if let Ok(mut transcribed) = model.transcribe(input) {
                                while let Some(transcribed) = transcribed.next().await {
                                    if transcribed.probability_of_no_speech() < 0.90 {
                                        let text = transcribed.text();
                                        document_engine.write().await.add(text).await.unwrap();
                                    }
                                }
                            }
                        }
                    })
            });
        }
        // ANCHOR_END: transcribe_audio
    }

    // ANCHOR: create_chat
    let mut model = Llama::new_chat();
    let mut chat = Chat::builder(&mut model).with_system_prompt("The assistant help answer questions based on the context given by the user. The model knows that the information the user gives it is always true.").build();
    // ANCHOR_END: create_chat

    // ANCHOR: rag
    loop {
        // Ask the user for a question
        let user_question = prompt_input("\n> ").unwrap();

        // Search for relevant context in the document engine
        let mut engine = document_engine.write().await;
        let context = {
            let context = engine.search(&user_question, 5).await;
            let context = context
                .iter()
                .take(5)
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            context.join("\n")
        };

        // Format a prompt with the question and context
        let prompt = format!(
            "Here is the relevant context:\n{context}\nGiven that context, answer the following question:\n{user_question}"
        );

        // Display the prompt to the user for debugging purposes
        println!("{}", prompt);

        // And finally, respond to the user
        let output_stream = chat.add_message(prompt).await.unwrap();
        print!("Bot: ");
        output_stream.to_std_out().await.unwrap();
    }
    // ANCHOR_END: rag
}
