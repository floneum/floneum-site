use kalosm::vision::ModelLoadingProgress;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    {
        // ANCHOR: create_embedding_model
        use kalosm::language::*;

        let bert = Bert::new().await?;
        // ANCHOR_END: create_embedding_model

        // ANCHOR: create__embedding_model_with_loading_handler
        let bert = Bert::builder()
            .build_with_loading_handler(|progress| match progress {
                ModelLoadingProgress::Downloading {
                    source,
                    start_time,
                    progress,
                } => {
                    let progress = (progress * 100.0) as u32;
                    let elapsed = start_time.elapsed().as_secs_f32();
                    println!("Downloading file {source} {progress}% ({elapsed}s)");
                }
                ModelLoadingProgress::Loading { progress } => {
                    let progress = (progress * 100.0) as u32;
                    println!("Loading model {progress}%");
                }
            })
            .await?;
        // ANCHOR_END: create__embedding_model_with_loading_handler

        // ANCHOR: create_embeddings
        let text = "Hello, world!";
        let embeddings = bert.embed(text).await?;
        println!("{:?}", embeddings);
        // ANCHOR_END: create_embeddings
    }

    {
        // ANCHOR: create_embedding_database
        use kalosm::language::*;

        // Create database connection
        let db =
            surrealdb::Surreal::new::<surrealdb::engine::local::RocksDb>("./db/temp.db").await?;

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await?;

        // Create a document table
        let document_table = db
            .document_table_builder("documents")
            // Store the embedding database in the ./db/embeddings.db file
            .at("./db/embeddings.db")
            .build()
            .await?;
        // ANCHOR_END: create_embedding_database

        // ANCHOR: extend_database
        let nyt = RssFeed::new(Url::parse(
            "https://rss.nytimes.com/services/xml/rss/nyt/US.xml",
        )?);

        // Fetch the documents from the feed
        let documents = nyt.into_documents().await?;
        // And insert them into the database
        for document in documents {
            document_table.insert(document).await?;
        }
        // ANCHOR_END: extend_database

        // ANCHOR: search_database
        loop {
            let user_question = prompt_input("Query: ")?;
            let user_question_embedding = document_table
                .embedding_model()
                .embed(&user_question)
                .await?;

            println!(
                "vector: {:?}",
                document_table
                    .select_nearest(user_question_embedding, 5)
                    .await?
            );
        }
        // ANCHOR_END: search_database
    }
}
