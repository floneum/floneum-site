let model = WhisperBuilder::default()
    .with_source(WhisperSource::MediumEn)
    .build()?;

let db = Surreal::new::<RocksDb>("./db/temp.db").await.unwrap();

db.use_ns("test").use_db("test").await.unwrap();

let document_table = Arc::new(db.document_table_builder("documents").at("./db/embeddings.db").build().unwrap());

let document_table_clone = document_table.clone();
std::thread::spawn(move || tokio::runtime::Runtime::new().unwrap().block_on(async move {
    loop {
        let input = MicInput::default().record_until(Instant::now() + Duration::from_secs(30)).await.unwrap();

        if let Ok(mut transcribed) = model.transcribe(input) {
            while let Some(transcribed) = transcribed.next().await {
                document_table_clone.insert(transcribed.text().into_document().await.unwrap()).await.unwrap();
            }
        }
    }
}));

let mut model = Llama::new_chat();
let mut chat = Chat::builder(&mut model).with_system_prompt("The assistant help answer questions based on the context given by the user. The model knows that the information the user gives it is always true.").build();

let context = document_table.select_nearest(&prompt_input("\n> ").unwrap(), 5).await?.into_iter().map(ToString::to_string).collect::<Vec<_>>().join("\n");

let output_stream = chat.add_message(format!("Here is the relevant context:\n{context}\nGiven that context, answer the following question:\n{user_question}")).await