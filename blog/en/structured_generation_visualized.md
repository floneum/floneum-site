# Structured Generation Visualized

![Structured Generation Visualized](/assets/structured_generation_visualized.png)

First some background:
- Tokens
- LLMs
- Grammars/Incremental parsing

Structured Generation Visualized

Text is a universal format for data. Data in text form is all over the web. Communication happens in JSON, code is written in . Because LLMs are trained on a giant corpus of web text, they can generally understand and write text in a machine readable format like JSON.


I want 100 different random characters with reasonable names, descriptions, and ages. Instead of creating each character individually, what if we ask a small LLM to generate JSON for each character?


We need JSON in this format:

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

We can use the [Kalosm](https://github.com/floneum/floneum/tree/main/interfaces/kalosm) library to generate text with the phi-3-mini-4k-instruct model. We will use the `Task` struct to create a task that streams unstructured text into stdout:

```rust
// Cargo.toml
// [dependencies]
// kalosm = { version = "0.3", features = ["language", "metal"] }
// tokio = { version = "1.37.0", features = ["full"] }

use kalosm::language::*;

#[tokio::main]
async fn main() {
    // Create a new model. We are using the Phi-3 model which is small and focused on reasoning tasks.
    let model = Llama::builder()
        .with_source(LlamaSource::phi_3_mini_4k_instruct())
        .build()
        .await
        .unwrap();

    // Create a task that generates text for a character
    let task = Task::builder("You generate data in a JSON format").build();

    // Run the task
    let result = task.run(r#"Generate a character with this format: { "name": string, "description": string, "metadata": { "age": number, "height_cm": number, "weight_kg": number, "hair_color": string, "eye_color": string } }"#, &model);

    // Stream the text into stdout
    result.to_std_out().await.unwrap();
}
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

Instead of just telling the LLM about the format we want, what if we force it to generate text that conforms to the format?

## Token Generation

First, lets dive into a bit of background on LLMs. Large language models are trained on a massive corpus of text. Instead of reading characters, or words in a sentence, the LLM is trained to read chunks of text called tokens. On average each token is about 2/3 of a word, but depending on the word it could be the entire word or a single character.


```inject-dioxus
TokenizationVisualization {}
```


To generate text, LLMs assign a probability to each token and picks a token with a high probability. Picking a token from the list of probabilities is called sampling.

## Incremental Parsing

Determining exactly what sequences are valid can be more difficult than it first appears. We need a parser that:
1) Incrementally parses new text in a way we can roll back. If the current tokens are `{"age":` we need to be able to try adding `a` and if it fails, roll back to `{"age"`. It shouldn't need to re-parse the entire string for every one of the `128,000` possible new tokens.
2) Fails fast. If a new token is invalid, we need to know that immediately. We can't batch up several new tokens and try to parse them all at once.

It turns out Regular Expressions are very well suited for this task. A regular expression can be represented as a finite state machine. Each state in the machine can be stored and restored easily.

```rust
// Cargo.toml
// [dependencies]
// kalosm = { version = "0.3", features = ["language", "metal"] }
// tokio = { version = "1.37.0", features = ["full"] }

use kalosm::language::*;

#[tokio::main]
async fn main() {
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
    let result = task.run(r#"Generate a character with this format: { "name": string, "description": string, "metadata": { "age": number, "height_cm": number, "weight_kg": number, "hair_color": string, "eye_color": string } }"#, &model);

    // Stream the text into stdout
    result.to_std_out().await.unwrap();
}
```

```json
{ "name": "Evelyn Archer", "description": "A cunning and resourceful detective with a sharp eye for detail who has solved numerous complex cases in the bustling city of New York during her prime years as an investigator at midlife crisis stage when she starts to question life choices leading up until now that age is just another number", "metadata": { "age": 45, "height_cm": 168, "weight_kg": 70, "hair_color": "Auburn", "eye_color": "Hazel" } }
```

## Constrained Augmented Sampling

Instead of just picking a token with the highest probability, we can filter the probabilities to only include tokens that are valid for our format. Even if you choose completely random options from the list of probabilities, the LLM will still generate text that conforms to the format.

From the LLM perspective, it can only "choose" from the valid tokens. Something like this:

```inject-dioxus
StructuredGenerationVisualization {}
```

## Accelerated Structured Generation

You may have noticed that some of the choices the LLM makes only have one valid next token. We don't actually need to run the LLM at all for these choices. Instead, we can just choose the next token directly. Now the LLM only chooses valid tokens where there is an interesting choice to make:

```inject-dioxus
StructuredGenerationAcceleratedVisualization {}
```

Instead of choosing each one of the ~16 tokens, we only need to choose between the ~4 important tokens.

## Sampler Aware Structured Generation

Right now, we are running the parser for every single one of the 128,000 tokens every time we add a new token to the sequence. This is very inefficient. Instead, we can take advantage of the structure of the sampler to only run the parser for the tokens that could actually be sampled.


There are generally a few steps between the token probabilities after constraints and the token that gets chosen. Each of those steps that modify the probabilities of the tokens is called a sampler. One common sampler is the top-k sampler which only samples the top k tokens with the highest probability. Here is what that could look like if we only keep the top 2 tokens (k=2):

![Top-k Sampling](/assets/top_k_sampling.png)

> Generally k is larger, but much smaller than the number of tokens in the model. The default in kalosm if 64.

We can combine the top-k sampler with constrained generation by only looking for the top k valid tokens after the constraints have been applied. Once we have the most probable k tokens, we can stop running the constraints against tokens at all. Since the LLM knows about the constraints, the valid tokens tend to have a very high probability which means we can skip parsing the majority of the 128,000 tokens.

![Top-K Accelerated Structured Generation](/assets/top_k_accelerated_structured_generation.png)
