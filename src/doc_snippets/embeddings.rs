#[tokio::main]
async fn main() {
    {
        // ANCHOR: create_embedding_model
        use kalosm::{language::*};

        let bert = Bert::default();
        // ANCHOR_END: create_embedding_model

        // ANCHOR: create_embeddings
        let text = "Hello, world!";
        let embeddings = bert.embed(text).await.unwrap();
        println!("{:?}", embeddings);
        // ANCHOR_END: create_embeddings
    }

    {
        // ANCHOR: create_embedding_database
        use kalosm::{language::*, *};

        // Create database connection
        let db = surrealdb::Surreal::new::<surrealdb::engine::local::RocksDb>("./db/temp.db").await.unwrap();

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await.unwrap();

        // Create a document table
        let mut document_table = db
            .document_table_builder("documents")
            // Store the embedding database in the ./db/embeddings.db file
            .at("./db/embeddings.db")
            .build()
            .unwrap();
        // ANCHOR_END: create_embedding_database

        // ANCHOR: extend_database
        let nyt = RssFeed::new(
            Url::parse("https://rss.nytimes.com/services/xml/rss/nyt/US.xml").unwrap(),
        );

        // Fetch the documents from the feed
        let documents = nyt.into_documents().await.unwrap();
        // And insert them into the database
        for document in documents {
            document_table.insert(document).await.unwrap();
        }
        // ANCHOR_END: extend_database

        // ANCHOR: search_database
        loop {
            let user_question = prompt_input("Query: ").unwrap();
            let user_question_embedding = document_table
                .embedding_model_mut()
                .embed(&user_question)
                .await
                .unwrap();

            println!(
                "vector: {:?}",
                document_table
                    .select_nearest_embedding(user_question_embedding, 5)
                    .await
                    .unwrap()
            );
        }
        // ANCHOR_END: search_database
    }
}
