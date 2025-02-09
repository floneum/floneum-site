# Custom parsers

If you need more control over the constraints the LLM follows, you can define your own parsers. 

## Defining Constraints

Kalosm provides a set of parsers that can be combined to define constraints. The following base parsers are available:

- `LiteralParser`: Matches a literal string.
- `IntegerParser`: Matches an integer (along with parsers for each rust integer type).
- `FloatParser`: Matches a float.
- `StringParser`: Matches a string.
- `SeparatorParser`: Matches any number of items separated by a separator.
- `IndexParser`: Matches any of a set of parsers and returns the index of the matched parser.
- `StopOn`: Matches anything until a literal.
- `WordParser`: Matches a single word.
- `VecParser`: Matches a vector of items.

And you can combine them using the following combinators:

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

Once you have defined a parser, you can generate text that adheres to the constraints defined by the parser. You can call `with_constraints` on a text stream or chat stream to force the model to adhere to the constraints defined by the parser:

```rust
{{#include src/doc_snippets/structured.rs:streaming_text}}
```
