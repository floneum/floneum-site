# Constrained Generation in Kalosm

## Overview

Language models can be incredibly powerful tools for difficult to define tasks. However, in some cases, it is necessary to constrain the output of a language model to a specific pattern. For example, you may want to generate text in a JSON format. Kalosm provides a powerful mechanism for constrained generation that allows you to define a set of constraints and generate text that adheres to those constraints.


## Defining Constraints

The simplest way to define constraints for structured generation is to use a derive a parser with the `Parse` trait and derive a description of the constraints with the `Schema` trait. The combination of these two traits lets you efficiently describe and parse a Rust type as json:

```rust
{{#include src/doc_snippets/structured.rs:derive_parser}}
```

> If you need more control over the structure the LLM generates, you can define a [custom parser](./custom_parser.md).

## Generating Text

Once you have defined a parser, you can generate text that adheres to the constraints defined by the parser. We can use a `Task` to generate text that follows the format we defined. The task will constrain the LLM to always generate text that matches the format:

```rust
{{#include src/doc_snippets/structured.rs:streaming_text}}
```

## Conclusion

Constrained generation in Kalosm enables the generation of text that follows specific grammatical rules or patterns. By leveraging the Kalosm library, developers can create sophisticated language models tailored to their application's requirements. The example code showcases how to use Kalosm to generate text with constraints, providing a foundation for building more advanced natural language generation systems.
