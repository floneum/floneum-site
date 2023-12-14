#[tokio::main]
async fn main() {
    {
        // ANCHOR: create_embedding_model
        use kalosm::language::*;

        let mut bert = Bert::default();
        // ANCHOR_END: create_embedding_model

        // ANCHOR: create_embeddings
        let text = "Hello, world!";
        let embeddings = bert.embed(text).await.unwrap();
        println!("{:?}", embeddings);
        // ANCHOR_END: create_embeddings
    }

    {
        // ANCHOR: create_embedding_database
        use kalosm::{language::*, prompt_input};

        let mut database = DocumentDatabase::new(
            Bert::builder().build().unwrap(),
            ChunkStrategy::Sentence {
                sentence_count: 1,
                overlap: 0,
            },
        );
        // ANCHOR_END: create_embedding_database

        // ANCHOR: extend_database
        let nyt = RssFeed::new(
            Url::parse("https://rss.nytimes.com/services/xml/rss/nyt/US.xml").unwrap(),
        );

        database.extend(nyt).await.unwrap();
        // ANCHOR_END: extend_database

        // ANCHOR: search_database
        loop {
            let user_question = prompt_input("Query: ").unwrap();

            println!(
                "vector: {:?}",
                database
                    .search(&user_question, 5)
                    .await
                    .iter()
                    .collect::<Vec<_>>()
            );
        }
        // ANCHOR_END: search_database
    }
}
