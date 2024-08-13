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
    let _validator = states
        .then(LiteralParser::from(", "))
        .repeat(5..=5)
        .then(LiteralParser::from("\n"));

    // ANCHOR_END: create_parser

    // ANCHOR: regex_parser

    // You can also use a regex to match the same pattern. However, you will not get a parsed result once the generator is finished.
    let validator = RegexParser::new(r"((Alaska|Delaware|Florida|Georgia|Hawaii), ){5}\n").unwrap();

    // ANCHOR_END: regex_parser

    // ANCHOR: streaming_text
    let llm = Phi::v2().await.unwrap();
    let mut structured = llm.stream_structured_text("A state that starts with A", validator);

    structured.to_std_out().await.unwrap();

    println!("Result: {:?}", structured.await.unwrap());
    // ANCHOR_END: streaming_text
}
