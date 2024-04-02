# Kalosm v0.2.0 Tasks, Evaluation, Prompt Auto-Tuning, Regex Validation, Surreal Database Integration, RAG improvements, Performance Improvements, and More!

We're excited to announce the release of Kalosm v0.2.0! This release includes a number of new features, improvements, and bug fixes including:

- Tasks and Agents
- Task Evaluation
- Prompt Auto-Tuning
- Regex Validation
- Surreal Database Integration
- RAG improvements
- Performance Improvements
- New Models

## Tasks and Agents!

Kalosm now includes utilities for running, evaluating, and improving tasks and agents.

Let's build a simple task and agent to demonstrate the new functionality.

```rust
use kalosm::language::*;

#[tokio::main]
async fn main() {
	// You can use tasks with llama, mistral or phi models.
    let llm = Phi::new_chat();

    // Now we can create a task. We will create a simple assistant that identifies keywords in a sentence.
    let task = Task::builder(&llm, "You are an assistant who identifies keywords in a sentence. When identifying keywords, you will output the keywords in a comma-separated list.")
		// You can optionally add constraints to the task.
        // accept 2-5 keywords of 2-15 characters each
        .with_constraints(RegexParser::new(r#"The keywords are:([ a-z]{2,15},){2,5}"#).unwrap())
		/// You can also add examples to the task to help the agent learn how to solve math problems.
        .with_example("What is the weather like in New York?", "The keywords are: weather, new york, ")
        .with_example("What is the capital of France?", "The keywords are: capital, france, ")
        .build();

    // The first time we use the task, it will load the model and prompt which will take a bit longer.
    task.run("What is the temperature in Chicago?", &llm)
        .await
        .unwrap()
        .to_std_out()
        .await
        .unwrap();
}
```

Or you can use more complex constraints to 

```rust
// First, we need to create a Llama instance. We will use the default chat model (open chat) for this example. Tasks work with chat and text generation models.
let mut llm = Llama::new_chat();

// Next (optionally), we can define constraints for the task. In this example, we will use a regex to validate the output of the task.
let constraints =
	RegexParser::new(r"(Step \d: \d+ [+\-*/] \d+ = \d+\n){1,3}Output: \d+").unwrap();

// Now we can create a task. We will create a simple math problem solving task.
let task = Task::builder(&llm, "You are an assistant who solves math problems. When solving problems, you will always solve problems step by step with one step per line. Once you have solved the problem, you will output the result in the format 'Output: <result>'.")
	.with_constraints(constraints)
	// You can also add examples to the task to help the agent learn how to solve math problems.
	.with_example("What is 1 + 2?", "Step 1: 1 + 2 = 3\nOutput: 3")
	.with_example("What is 3 + 4?", "Step 1: 3 + 4 = 7\nOutput: 7")
	.with_example("What is (4 + 8) / 3?", "Step 1: 4 + 8 = 12\nStep 2: 12 / 3 = 4\nOutput: 4")
	.build();

// The first time we use the task, it will load the model and prompt which will take a bit longer.
task.run("What is 2 + 2?", &llm)
	.await
	.unwrap()
	.to_std_out()
	.await
	.unwrap();
```

Tasks can efficiently reuse the session between runs, which can significantly speed up the process:

```
question 1
Step 1: 2 + 2 = 4
Output: 4.
first question took: 18.371939709s
question 2
Step 1: 4 + 4 = 8
Output: 8
second question took: 5.723529959s
question 3
Step 1: 7 + 5 = 12
Step 2: 12 / 2 = 6
Output: 6
third question took: 9.303650625s
```

The third question required a more complex calculation and took more tokens to solve, but the time to solve the question was still significantly faster than the first question. The session from the first question was reused for the second and third questions which made the second and third questions run faster.

- **Evaluation Abstraction:** Introducing an evaluation abstraction, providing enhanced functionality. ([#113](https://github.com/floneum/floneum/pull/113))
```
+-----------------+-------+
| Statistic       | Value |
+=========================+
| Mean            | 0.75  |
|-----------------+-------|
| Median          | 0.77  |
|-----------------+-------|
| Min             | 0.49  |
|-----------------+-------|
| Max             | 0.94  |
|-----------------+-------|
| 25th Percentile | 0.67  |
|-----------------+-------|
| 75th Percentile | 0.81  |
+-----------------+-------+
+------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------+
| Expected Output                                                                          | Actual Output                                                                             | Score              |
+===========================================================================================================================================================================================================+
| What are Floneum plugins?                                                                | What is designed to support an expanding ecosystem of plugins?                            | 0.49 (low outlier) |
|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|
| What is supervised learning in machine learning?                                         | What is required for machine learning models in order to make accurate predictions?       | 0.50 (low outlier) |
|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|
| What distinguishes open-source software from proprietary software?                       | What type of access does open source software provide for its users?                      | 0.64               |
|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|
| How does blockchain contribute to the security of cryptocurrencies?                      | What is the advantage of cryptocurrency over traditional currencies in terms of security? | 0.65               |
|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|
| What are the tradeoffs of using chat GPT?                                                | What is an example of how using ChatGPT may become challenging in certain situations?     | 0.65               |
|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|
| What is the relationship between DevOps, continuous integration and continuous delivery? | What is the main goal of DevOps?                                                          | 0.67               |
|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|
| ... 18 more                                                                              |                                                                                           | 0.80 (average)     |
+------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------+
| Score Histogram        |
| 0.00 - 0.10:           |
| 0.10 - 0.20:           |
| 0.20 - 0.30:           |
| 0.30 - 0.40:           |
| 0.40 - 0.50: *         |
| 0.50 - 0.60: *         |
| 0.60 - 0.70: ******    |
| 0.70 - 0.80: ********* |
| 0.80 - 0.90: *****     |
| 0.90 - 1.00: **        |
```

- **Prompt Auto-Tuning:** Prompts can now be automatically tuned for better performance. ([#132](https://github.com/floneum/floneum/pull/132))

Lets take a look at the new prompt auto-tuning feature with an example. As part of the [RAG improvements](#Improved-Chunking-Strategies), kalosm includes a task that generates hypothetical questions about a text for an embedding model that can be used to find similar documents based on the meaning of the text for the  section. We can tune that task to find the best examples for the task with a `PromptAnnealer`:

```rust
const EXAMPLES: &[(&str, &str)]= &[
    ("An example input to the task", "An example output to the task"),
    // ...more examples
];

let llm = Phi::v2();
const PREFIX: &str = "Questions that are answered by the previous text: ";
const QUESTION_STARTERS: [&str; 9] = [
    "Who", "What", "When", "Where", "Why", "How", "Which", "Whom", "Whose",
];
let constraints = LiteralParser::new(PREFIX).then(
    IndexParser::new(
        QUESTION_STARTERS
            .iter()
            .copied()
            .map(LiteralParser::new)
            .collect::<Vec<_>>(),
    )
    .then(StopOn::new("?").filter_characters(
        |c| matches!(c, ' ' | '?' | 'a'..='z' | 'A'..='Z' | '0'..='9' | ','),
    ))
    .repeat(1..=5),
);
let task = Task::builder("You generate hypothetical questions that may be answered by the given text. The questions restate any information necessary to understand the question")
    .with_constraints(constraints);
let mut annealing = kalosm::PromptAnnealer::builder(&llm, EXAMPLES, task)
    .with_initial_temperature(0.6)
    .with_initial_choice_range(1..4)
    .build()
    .await;

let result = annealing.run().await;

println!("Result: {:?}", result);
```

Here is the best set of examples that the prompt annealer found for the task:

| Input | Output |
| ----- | ------ |
| While traditional databases rely on a fixed schema, NoSQL databases like MongoDB offer a flexible structure, allowing you to store and retrieve data in a more dynamic way. This flexibility is particularly beneficial for applications with evolving data requirements. | "How does MongoDB differ from traditional databases? |
| Blockchain technology, beyond cryptocurrencies, is being explored for applications like smart contracts. Smart contracts are self-executing contracts with the terms of the agreement directly written into code. | "How is blockchain technology utilized in the concept of smart contracts? |

Feeding those two examples into the task achieves a similarity score of 0.71 for all of the other examples compared to choosing two random examples from the task which only achieves a similarity score of  0.62 for all of the other examples.

## Regex Validation

Some feedback we got from the initial release of kalosm, was that constraints for constrained generation was too complex. Constraints in Kalosm serve two purposes:

1) Validation: Constraints can be used to validate the output of the model. The model will only output text that can be parsed by the constraints. This lets you ensure that the output of the model is in the format you expect.


For example, you may want to force the model response to always start with a prefix that guides the model:
```rust
use kalosm::language::*;

#[tokio::main]
async fn main() {
    let llm = Llama::default();
    let prompt = "Generate a list of 10 words you use to describe yourself: ";

    let validator = LiteralParser::new("(Responding as a pirate) ").then(OneLine);
    let stream = llm.stream_structured_text(prompt, validator).await.unwrap();

    stream.to_std_out().await.unwrap();
}
```

2) Parsing: Constraints can be used to parse the output of the model. This can be extremely useful when you want to generate a specific structure from an LLM without writing separate logic for validation and parsing.

For example, you may want to generate a list of 10 numbers:
```rust
use kalosm::language::*;

#[tokio::main]
async fn main() {
    let llm = Llama::default();
    let prompt = "Prime numbers: ";

    let validator = <[f32; 10] as HasParser>::new_parser();
    let words = llm.stream_structured_text(prompt, validator).await.unwrap();

    let result: [f32; 10] = words.result().await.unwrap();
    println!("Prime numbers: {:?}", result);
}
```

If you only need to validate the output of the model, the existing constraints can be more complex than what you need. In this release, we've added support for regex validation. This makes it easier to validate the output of the model without handling parsing:

```rust
use kalosm::language::*;

#[tokio::main]
async fn main() {
    let llm = Llama::default();
    let prompt = "Generate a list of 10 words you use to describe yourself: ";

    let validator = RegexParser::new(r"\(Responding as a pirate\) ([a-z]{1,10}, ){10}").unwrap();
    let stream = llm.stream_structured_text(prompt, validator).await.unwrap();

    stream.to_std_out().await.unwrap();
}
```

## Surreal Database Integration

Vector databases can be very useful when combined with LLMs. They can be used to store and retrieve similar documents based on the meaning of the text, not just the words used. However, vector databases only handle a very limited number of use cases. In this release, we've added support for Surreal DB for more traditional database use cases. Surreal DB can be embedded into your application and used to store and retrieve data locally as well as over the network.

Kalosm 0.2 allows you to create tables within Surreal DB that are indexed by vectors. You can then insert documents (or other embeddings) into the table and query the table for similar documents based on the meaning of the text.

Let's take a look at how you can use the Surreal DB integration to store and retrieve similar documents based on the meaning of the text:

```rust
use comfy_table::{Cell, Color, Row, Table};
use kalosm::language::*;
use kalosm::*;
use std::path::PathBuf;
use surrealdb::{engine::local::RocksDb, Surreal};

let exists = std::path::Path::new("./db").exists();

// Create or open a new database at ./db/temp.db
let db = Surreal::new::<RocksDb>("./db/temp.db").await.unwrap();

// Select a specific namespace / database within the database
db.use_ns("test").use_db("test").await.unwrap();

// Create a new document table that uses arroy for fast vector search
let document_table = db
    .document_table_builder("documents")
    // Store the embedding database in the same directory as the document table
    .at("./db/embeddings.db")
    .build()
    .unwrap();

// If the database doesn't exist, create it and insert some documents
if !exists {
    std::fs::create_dir_all("documents").unwrap();
    // Load some files from a directory
    let documents = DocumentFolder::try_from(PathBuf::from("./documents")).unwrap();

    // Convert the folder into a vector of documents
    let documents = documents.into_documents().await.unwrap();
    for document in documents {
        // And insert the documents into the table (this will automatically chunk and embed the documents before they are inserted into the table and vector database)
        document_table.insert(document).await.unwrap();
    }
}

// Now we can query the table for similar documents based on the meaning of the text
loop {
    // Get a query from the user and embed that query into a vector
    let user_question = prompt_input("Query: ").unwrap();
    let user_question_embedding = document_table
        .embedding_model_mut()
        .embed(&user_question)
        .await
        .unwrap();

    // Select the 5 most similar documents to the user's query
    let nearest_5 = document_table
        .select_nearest_embedding(user_question_embedding, 5)
        .await
        .unwrap();

    // Display the results in a formatted table with colors based on the distance from the query
    let mut table = Table::new();
    table.set_header(vec!["Score", "Value"]);

    for result in nearest_5 {
        let mut row = Row::new();
        let color = if result.distance < 0.25 {
            Color::Green
        } else if result.distance < 0.75 {
            Color::Yellow
        } else {
            Color::Red
        };
        row.add_cell(Cell::new(result.distance).fg(color))
            .add_cell(Cell::new(result.record.body()[0..50].to_string() + "..."));
        table.add_row(row);
    }

    println!("{}", table);
}
```

## RAG improvements

RAG (Retrieval-Augmented Generation) is a powerful tool for generating text with up-to-date or proprietary information. Retrieval-augmented generation generally follows the following steps:

1) Gather context from some local files, your database, or web data. In kalosm, you can retrieve data from any source that implements [`IntoDocument`](https://docs.rs/kalosm/0.2.0/kalosm/language/trait.IntoDocument.html) or [`IntoDocuments`](https://docs.rs/kalosm/0.2.0/kalosm/language/trait.IntoDocuments.html). You can gather your sources from [local documents](https://docs.rs/kalosm/0.2.0/kalosm/language/struct.DocumentFolder.html), a [search term](https://docs.rs/kalosm/0.2.0/kalosm/language/struct.SearchQuery.html), [specific web page](https://docs.rs/kalosm/0.2.0/kalosm/language/struct.Url.html), an [RSS feed](https://docs.rs/kalosm/0.2.0/kalosm/language/struct.RssFeed.html), or even a [custom web crawler](https://docs.rs/kalosm/0.2.0/kalosm/language/enum.Page.html#method.crawl).
2) Insert that context into a searchable database. Kalosm includes a [vector database](https://docs.rs/kalosm/0.2.0/kalosm/language/struct.VectorDB.html) that can be used to store and retrieve similar documents.

Vector databases use an embedding model which generates a vector for a chunk of text (typically smaller than the entire document). The vector is then stored in the database. When you want to retrieve similar documents, you can embed a query and search for similar vectors in the database.

The vectors represent the meaning of the text, so you can search for similar documents based on the meaning of the text, not just the words used.

3) Use the context to generate text. You can find text similar to the question or a search generated by the LLM and then generate a response based on the context.

In this release, we've made several improvements to RAG! ([#126](https://github.com/floneum/floneum/pull/126))

### Improved Chunking Strategies

When you insert a document into a vector database, it needs to be split into smaller chunks before the text is embedded. The chunks you choose can have a significant impact on the performance of the results you get from the vector database. In this release, we've added two new chunking strategies to the vector database:

- Hypothetical Questions

Instead of generating embeddings based on the content of the document, this chunking strategy generates embeddings based on hypothetical questions generated about the document. This can be extremely useful when building a chatbot that needs to find context that is relevant to a question.

For example, if you have a document about the history of the United States, you can generate hypothetical questions like "What is the capital of the United States?" and "Who was the first president of the United States?" and then generate embeddings based on those questions.

Then if you query the vector database with a question like "Who was the leader of the US?" you can find the document about the history of the United States.

> Notice that the question "Who was the leader of the US?" doesn't contain many of the same words as the hypothetical questions, but it does convey a similar meaning, so the vector database can still find the relevant document.

- Summaries

This chunking strategy generates embeddings based on the summary of the document. This can be useful when you have a large document and you want to find similar documents based on the main points of the document.

Generating embeddings based on the summary of the document can create better embeddings that contain more information about the document than embeddings that only contain one small chunk of the document.

### Incremental Indexing

In addition to the new chunking strategies, we've also added support for incremental indexing. This means you can add new documents to the vector database without having to recreate the entire database. This can be extremely useful when you have a large database or you have constantly updating context you want to provide to your LLM.

Kalosm's Vector database is now backed by [arroy](https://github.com/meilisearch/arroy), a space-efficient and incrementally indexed vector database backed by MeiliSearch!

## Performance Improvements

The llama implementation has been rewritten and optimized for better performance and modularity. The new implementation is now 7-25% faster than the previous version. In future releases, we plan to add support for fine tuning models and training new heads for existing models. ([#122](https://github.com/floneum/floneum/pull/122))

Language models like Llama and Phi output probabilities for each token in the vocabulary. To generate text you need to sample from the probability distribution. Sampling from the probability distribution can be slow, especially with large vocabularies. Kalosm 0.2 uses an optimization introduced in [llm-samplers](https://github.com/KerfuffleV2/llm-samplers/pull/9) to only sample top 512 tokens. This optimization can make sampling up to 2x faster. ([#123](https://github.com/floneum/floneum/pull/123))

Large sections of text that are static within a constraint in structured generation is now loaded in a batch which can significantly speed up the process.

For example if you have the constraints:

```rust
let constraints = RegexParser::new(r#"(The title of the book is [a-z]{2,15}\n)*"#).unwrap();
```

The text "The title of the book is " will be loaded in a batch instead of one token at a time. Batched loading has been restored in constrained generation. ([#131](https://github.com/floneum/floneum/pull/131))

## New Models

Kalosm 0.2 adds support for several new models, including:

- **Dolphin Phi v2** A tiny chat model
- **Solar-11b Models** A set of models for chat, text, and code generation
- **Tiny Llama 1.0** A tiny set of models for chat, and text text generation

## Full Changelog
For a detailed list of changes between v0.1.0 and v0.2.0, please see the [full changelog](https://github.com/floneum/floneum/compare/v0.2.0...v0.2.0-kalosm).

I hope you enjoy using Kalosm v0.2.0! Your feedback is invaluable to us, so please don't hesitate to share your thoughts and report any issues you encounter.

## What's next?

In the next release, we plan to add support for fine tuning models and training new heads for existing models. We also plan to continue improving the performance of the language models and adding support for more models.

If any of those features sound interesting or you want to propose a new feature, consider contributing on [Github](https://github.com/floneum/floneum/tree/main/interfaces/kalosm).

If you are interested in building an application with Kalosm, [join the Discord](https://discord.gg/dQdmhuB8q5) and get involved with the community!
