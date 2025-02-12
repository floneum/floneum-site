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
    let regex = RegexParser::new(r#"((Alaska|Delaware|Florida|Georgia|Hawaii), ){5}\n"#).unwrap();
    // ANCHOR_END: regex_parser

    // ANCHOR: derive_parser
    #[derive(Clone, Debug, Parse, Schema)]
    struct Character {
        name: String,
        description: String,
        age: u8,
    }
    // ANCHOR_END: derive_parser

    // ANCHOR: streaming_text
    let llm = Llama::phi_3().await.unwrap();
    let task = llm
        .task("You generate realistic characters for a procedurally generated game.")
        .typed();

    let mut stream = task("Generate a character that is a wizard");
    stream.to_std_out().await.unwrap();

    let character: Character = stream.await.unwrap();
    println!("Result: {:?}", character);
    // ANCHOR_END: streaming_text
}
