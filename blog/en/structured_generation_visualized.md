# Structured Generation

![Structured Generation Visualized](./public/assets/structured_generation_visualized.png)

Text is the universal format for data. Computers communicate with JSON, code is written in text, and the end result is often rendered in HTML. LLMs are trained on a giant corpus of web text, so they should be excellent at understanding and writing formats like JSON right? Let's try asking a LLM to generate JSON for a character in this format:

```
{
    "name": string,
    "description": string,
    "metadata": {
        "age": number,
        "height": number (cm),
        "weight": number (kg),
        "hair_color": string,
        "eye_color": string,
    }
}
```

This should be pretty simple. Lets use a small model that is good at structured formats and reasoning called Phi-3-Mini. We can use the [Kalosm](https://github.com/floneum/floneum/tree/main/interfaces/kalosm) library to stream the text:

```rust
// Create a new model. We are using the Phi-3 model which is small and focused on reasoning tasks.
let model = Llama::builder()
    .with_source(LlamaSource::phi_3_mini_4k_instruct())
    .build()
    .await
    .unwrap();

// Create a task that generates text for a character
let task = Task::builder("You generate data in a JSON format").build();

// Run the task
let mut result = task.run(r#"Generate a character with this format: { "name": string, "description": string, "metadata": { "age": number, "height_cm": number, "weight_kg": number, "hair_color": string, "eye_color": string } }"#, &model);

// Stream the text into stdout
result.to_std_out().await.unwrap();
```

Running the program sometimes generates valid JSON with reasonable data:

```json
{
  "name": "Michael Thompson",
  "description": "A dedicated software developer with a passion for coding and innovation. Known amongst friends to be an avid reader, particularly of science fiction novels.",
  "metadata": {
    "age": 29,
    "height_cm": 180,
    "weight_kg": 75,
    "hair_color": "dark brown",
    "eye_color": "blue"
  }
}
```

But other times, it generates nonsense:

```json
{
   "name":"Emily Thompson",
    "description":"A talented graphic designer with an eye-catching style. Emily is known to be creative, friendly and dedicated at her job.",
     "metadata": { 
         "age":28,
          "height":165,"cm" :94,
           "weight":"60","kg",
            "hair_color":"blonde",
             "eye_color":"blue" }  
}
```

Ok, so just asking nicely isn't enough... The LLM knows something about the structure of JSON, but it isn't consistent enough to generate valid JSON every time.


We need some way of controlling what the LLM generates to make it fit our format. First, what exactly does the LLM generate?

## Token Generation

LLM see text in chunks of tokens. On average each token is about 2/3 of a word, but depending on the word it could be the entire word or a single character.


Lets take a look at what tokens look like from the perspective of the LLM. You can see what previous tokens the LLM has seen at the bottom of the visualization and a few of the possible next tokens on the right:

```inject-dioxus
TokenizationVisualization {}
```


To generate text, LLMs assign a probability to each token and picks a token with a high probability. Picking a token from the list of probabilities is called sampling.


We need to control what tokens the LLM picks after it assigns a probability to each token. Instead of picking the most likely token, we need to pick the best token that fits our format.

## Defining the Format

Ok, one piece left, we need define our format. We need a parser that:
1) Incrementally parses new text in a way we can roll back. If the current tokens are `{"age":` we need to be able to try adding `a` and if it fails, roll back to `{"age"`. If our current text is long, our parser shouldn't need to re-parse the entire history for every one of the `~128,000` possible new tokens
2) Fails immediately if a new token is invalid. We need to validate every token individually so we don't waste time trying a sequence of tokens and then walking back once we realize it's invalid

It turns out Regular Expressions are very well suited for this task. Every regular expression can be decomposed into a series of states and transitions (called a finite state machine). Each state is cheap to store, generally just a single index into a table. Each transition is cheap to compute, just a lookup into a table.


Here is a regular expression that matches our JSON schema:

```regex
\{ "name": "[A-Z][a-z]{1,10} [A-Z][a-z]{1,10}", "description": "[ A-Za-z]+", "metadata": \{ "age": \d{1,2}, "height_cm": \d{1,3}, "weight_kg": \d{1,3}, "hair_color": "[A-Z][a-z]+", "eye_color": "[A-Z][a-z]+" \} \}
```

## Constrained Augmented Sampling

Let's combine our format with the sampler. We can only sample tokens that fit our regex. Even if you choose completely random options from the list of probabilities, the LLM will still generate text that conforms to the format. From the perspective of the LLM, it looks something like this:

```inject-dioxus
StructuredGenerationVisualization {}
```

Lets see what our LLM generates with our new constraints:

```rust
// Create a new model. We are using the Phi-3 model which is small and focused on reasoning tasks.
let model = Llama::phi_3()
    .await
    .unwrap();

// Create a constraint that checks if the generated text is valid JSON
let constraint = RegexParser::new(r#"\{ "name": "[A-Z][a-z]{1,10} [A-Z][a-z]{1,10}", "description": "[ A-Za-z]+", "metadata": \{ "age": \d{1,2}, "height_cm": \d{1,3}, "weight_kg": \d{1,3}, "hair_color": "[A-Z][a-z]+", "eye_color": "[A-Z][a-z]+" \} \}"#).unwrap();
// Create a task that generates text for a character
let task = Task::builder("You generate data in a JSON format")
    .with_constraints(constraint)
    .build();

// Run the task
let mut result = task.run(r#"Generate a character with this format: { "name": string, "description": string, "metadata": { "age": number, "height_cm": number, "weight_kg": number, "hair_color": string, "eye_color": string } }"#, &model);

// Stream the text into stdout
result.to_std_out().await.unwrap();
```

The results are much more consistent. The LLM always generates valid JSON because of the constraints. It also generates json in a more precise format because we were able to specify everything about the format we wanted. It knows that the name is two words that each start with a capital letter, and the age can't be more than two digits.

```json
{ "name": "Evelyn Archer", "description": "A cunning and resourceful detective with a sharp eye for detail who has solved numerous complex cases in the bustling city of New York during her prime years as an investigator at midlife crisis stage when she starts to question life choices leading up until now that age is just another number", "metadata": { "age": 45, "height_cm": 168, "weight_kg": 70, "hair_color": "Auburn", "eye_color": "Hazel" } }
```

## Accelerated Structured Generation

You may have noticed that in some positions the LLM only has one valid token choice. We don't actually need to run the LLM in these cases. Instead, we can just choose the next token directly.


In practice, this acts a bit like autocomplete. Once the LLM starts generating a specific part of the grammar, the rest of it is filled for it:

```inject-dioxus
StructuredGenerationAcceleratedVisualization {}
```

Instead of choosing each one of the ~16 tokens, we only need to choose between the ~4 important tokens!


The LLM will still need to read the new text that got filled in, but that process is much faster than determining the probabilities for the next token.

## Beyond REGEX

REGEX is great for simple constraints, but it is both tedious to write and limited to simple patterns. There are some structures you just can't express with REGEX. For example, the general version of JSON itself!


JSON is recursive which means we need to store extra state to keep track of the current depth of the JSON that regex doesn't have.

### Deriving Parsers

If you don't need complete control over the syntax the model uses to generate json, you can just derive a parser for JSON from a type:

```rust
// Cargo.toml
// [dependencies]
// kalosm = { version = "0.3", features = ["language", "metal"] }
// tokio = { version = "1.37.0", features = ["full"] }

use kalosm::language::*;

// Define a type for the character
#[derive(Parse)]
struct Character {
    name: String,
    description: String,
    metadata: Metadata,
}

// Define a type for the metadata
#[derive(Parse)]
struct Metadata {
    age: u8,
    height_cm: u8,
    weight_kg: u8,
    hair_color: String,
    eye_color: String,
}

#[tokio::main]
async fn main() {
    // Create a new model. We are using the Phi-3 model which is small and focused on reasoning tasks.
    let model = Llama::phi_3()
        .await
        .unwrap();

    // Create a constraint that parses our character type
    let constraint = Character::new_parser();
    // Create a task that generates text for a character
    let task = Task::builder("You generate data in a JSON format")
        .with_constraints(constraint)
        .build();

    // Run the task
    let mut result = task.run(r#"Generate a character with this format: { "name": string, "description": string, "metadata": { "age": number, "height_cm": number, "weight_kg": number, "hair_color": string, "eye_color": string } }"#, &model);

    // Stream the text into stdout
    result.to_std_out().await.unwrap();

    // BONUS: Parsing is validation. If you derive a parser, you automatically get out parsed data!
    let parsed: Parsed = result.parse().await.unwrap();
}
```

### Creating Custom Parsers

If you need more control over what the model generates, you can create your own parser. Unlike regex, your parser can parse languages that [require context like html](https://github.com/ealmloff/html-parser):

```rust
use kalosm::language::*;

#[tokio::main]
async fn main() {
    let model = Llama::phi_3().await.unwrap();
    let task = Task::builder("The assistant generates plain HTML")
        .with_constraints(
            html_parser::Element::new_parser()
        )
        .build();

    let input = prompt_input("> ").unwrap();
    let mut output = task.run(input, &model);
    output.to_std_out().await.unwrap();
    let html: html_parser::Element = output.await.unwrap();
    println!();
    println!("{:#?}", html);
}
```

Because parsing runs in native rust code, you can parse even complex languages like html with constraints for elements, attributes and values in real time:

```inject-dioxus
HtmlStructuredGenerationAcceleratedVisualization {}
```

<!-- ![HTML Parser](./public/assets/html_parser.mp4) -->

## Sampler Aware Structured Generation

Right now, we are running the parser for every single one of the 128,000 tokens every time we add a new token to the sequence. That was fine for REGEX validation which is just a fancy look up table, but as our parsers get more complex, it starts to grind generation to halt. Instead of running the parser for every token, we can take advantage of the structure of the sampler to only run the parser for the tokens that could actually be sampled.


There are generally a few steps between the token probabilities after constraints and the token that gets chosen. Each of those steps that modify the probabilities of the tokens is called a sampler. One common sampler is the top-k sampler which only samples the top k tokens with the highest probability. Here is what that could look like if we only keep the top 2 tokens (k=2):

![Top-k Sampling](./public/assets/top_k_sampling.png)

> Generally k is larger, but much smaller than the number of tokens in the model. The default in kalosm if 64.

We can combine the top-k sampler with constrained generation by only looking for the top k valid tokens after the constraints have been applied. Once we have the most probable k tokens, we can stop running the constraints against tokens at all. Since the LLM knows about the constraints, the valid tokens tend to have a very high probability which means we can skip parsing the majority of the 128,000 tokens.

![Top-K Accelerated Structured Generation](./public/assets/top_k_accelerated_structured_generation.png)

## Conclusion


