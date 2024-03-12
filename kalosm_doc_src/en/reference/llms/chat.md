# Chat-based Interaction

 Chat is a simple but effective way to interact with a language model. This example demonstrates how to set up a chat-based interaction with a pirate-themed assistant using Kalosm.

## Setting up the Environment

Before we explore the code, ensure that you have the required dependencies. The example uses the `kalosm` and `tokio` crates. Make sure to add these dependencies to your project's `Cargo.toml` file:

```toml
[dependencies]
kalosm = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Example Code Explanation

Now, let's break down the example code step by step:

- Creating a Chat Language Model

Kalosm supports several chat variants of language models with the builder method introduced in [llms](../index.md) chapter. If you want a good default chat model, Kalosm also provides the `new_chat` method.

```rust
{{#include src/doc_snippets/chat.rs:create_chat_model}}
```

- Creating a Chat Instance

Now that we have a language model, we can create a chat instance using the builder pattern. The `with_system_prompt` method sets a system prompt that defines the behavior of the assistant. In this case, the assistant is configured to act like a pirate.

```rust
{{#include src/doc_snippets/chat.rs:create_chat_wrapper}}
```

Create a chat instance using the builder pattern. The `with_system_prompt` method sets a system prompt that defines the behavior of the assistant. In this case, the assistant is configured to act like a pirate.

4. **Starting the Conversation Loop:**

After you have created a chat instance, you can start the conversation loop. First, we need to get the users question from the terminal with `prompt_input`. Every time you add a message to the conversation with `add_message`, the assistant will start streaming a response. You can use the `to_std_out` method to print the response to the standard output.

```rust

```

## Conclusion

This example demonstrates how to set up a chat-based interaction with a pirate-themed assistant using Kalosm. More chat example code can be found in the [examples](https://github.com/floneum/floneum/tree/main/interfaces/kalosm/examples) folder.
