#[tokio::main]
async fn main() {
    // ANCHOR: create_parser
    use kalosm::language::*;

    // Create a list of parser for states
    let states = ["Alaska", "Delaware", "Florida", "Georgia", "Hawaii"];
    let states_parser = states
        .into_iter()
        .map(LiteralParser::from)
        .collect::<Vec<_>>();

    // Create a parser that tries to match each state
    let states = IndexParser::new(states_parser);

    // match a state, followed by a comma and a space, 5 times, and a newline
    let validator = states
        .then(LiteralParser::from(", "))
        .repeat(5..=5)
        .then(LiteralParser::from("\n"));

    // ANCHOR_END: create_parser

    // ANCHOR: streaming_text
    let mut llm = Phi::start().await;
    let (structured, result) = llm
        .stream_structured_text("A state that starts with A", validator)
        .await
        .unwrap()
        .split();

    structured.to_std_out().await.unwrap();

    println!("Result: {:?}", result.await.unwrap());
    // ANCHOR_END: streaming_text
}
