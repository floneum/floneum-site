# Constrained Generation in Kalosm

## Overview

Language models can be incredibly powerful tools for tasks that are difficult to define. However, in some cases, it is necessary to constrain the output of a language model to a specific set of rules or patterns. For example, you may want to generate text in a JSON format. Kalosm provides a powerful mechanism for constrained generation that allows you to define a set of constraints and generate text that adheres to those constraints.


Importing Dependencies:**
   ```rust
   use futures_util::stream::StreamExt;
   use kalosm_language::*;
   use kalosm_sample::*;
   use std::io::Write;
   use std::sync::Arc;
   use std::sync::Mutex;
   ```

   Import necessary libraries and dependencies, including `futures_util`, `kalosm_language`, and `kalosm_sample`.

2. **Initializing the Language Model:**
   ```rust
   let mut llm = Phi::start().await;
   ```

   Create an instance of the Kalosm language model (`Phi`) using the `start` method.

3. **Defining Constraints:**
   ```rust
   let prompt = "Five US states in central US are ";
   let states = [
       // List of US states
   ];
   let states_parser = states
       .into_iter()
       .map(LiteralParser::from)
       .collect::<Vec<_>>();
   let states = IndexParser::new(states_parser);
   let validator = states
       .then(LiteralParser::from(", "))
       .repeat(5..=5)
       .then(LiteralParser::from("\n"));
   let validator_state = validator.create_parser_state();
   ```

   Define a prompt and constraints. In this case, the constraint is a list of US states followed by a comma and repeated five times.

4. **Constrained Generation:**
   ```rust
   let mut words = llm
       .stream_structured_text_with_sampler(
           prompt,
           validator,
           validator_state,
           Arc::new(Mutex::new(GenerationParameters::default().sampler())),
       )
       .await
       .unwrap();
   ```

   Generate text that adheres to the defined constraints using the `stream_structured_text_with_sampler` method.

5. **Displaying the Output:**
   ```rust
   while let Some(text) = words.next().await {
       print!("{}", text);
       std::io::stdout().flush().unwrap();
   }
   ```

   Print the generated text to the console.

6. **Result Inspection:**
   ```rust
   println!("{:#?}", words.result().await);
   ```

   Print additional information about the generated text, such as parsing result.

7. **Unconstrained Generation:**
   ```rust
   let mut words = llm.stream_text(prompt).with_max_length(100).await.unwrap();
   while let Some(text) = words.next().await {
       print!("{}", text);
       std::io::stdout().flush().unwrap();
   }
   ```

   Generate text without constraints for comparison.

## Conclusion

Constrained generation in Kalosm enables the generation of text that follows specific grammatical rules or patterns. By leveraging the Kalosm library, developers can create sophisticated language models tailored to their application's requirements. The example code showcases how to use Kalosm to generate text with constraints, providing a foundation for building more advanced natural language generation systems.