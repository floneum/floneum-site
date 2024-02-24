# Constrained Generation in Kalosm

## Overview

Language models can be incredibly powerful tools for difficult to define tasks. However, in some cases, it is necessary to constrain the output of a language model to a specific pattern. For example, you may want to generate text in a JSON format. Kalosm provides a powerful mechanism for constrained generation that allows you to define a set of constraints and generate text that adheres to those constraints.


## Defining Constraints

Kalosm provides a set of parsers that can be used to define constraints. These parsers can be combined to create complex constraints. The following parsers are available:

- `LiteralParser`: Matches a literal string.
- `IntegerParser`: Matches an integer (along with parsers for each rust integer type).
- `FloatParser`: Matches a float.
- `StringParser`: Matches a string.
- `SeparatorParser`: Matches any number of items separated by a separator.
- `IndexParser`: Matches any of a set of parsers and returns the index of the matched parser.
- `StopOn`: Matches anything until a literal.
- `WordParser`: Matches a single word.
- `VecParser`: Matches a vector of items.

These parsers can be combined using the following combinators:

- `then`: Matches the first parser followed by the second parser.
- `or`: Matches the first parser or the second parser.
- `repeat`: Matches the parser a specified number of times.

In this example, we will create a parser that completes a sentence with only valid states by combining the `LiteralParser` and `IndexParser`:

```rust
{{#include src/doc_snippets/structured.rs:create_parser}}
```

If you don't care about the output of the parser, but you want the LLM to adhere to a specific structure, you can also use a `RegexParser` to match a regular expression:

```rust
{{#include src/doc_snippets/structured.rs:regex_parser}}
```

## Generating Text

Once you have defined a parser, you can generate text that adheres to the constraints defined by the parser. The `stream_structured_text` function takes a prompt and a parser and returns a stream of text that adheres to the constraints defined by the parser along with the results once generation is finished. The following example shows how to generate text using the parser defined above:

```rust
{{#include src/doc_snippets/structured.rs:streaming_text}}
```

## Conclusion

Constrained generation in Kalosm enables the generation of text that follows specific grammatical rules or patterns. By leveraging the Kalosm library, developers can create sophisticated language models tailored to their application's requirements. The example code showcases how to use Kalosm to generate text with constraints, providing a foundation for building more advanced natural language generation systems.
