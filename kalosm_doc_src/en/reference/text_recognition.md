# Text Recognition

Kalosm allows developers to extract text information from images using optical character recognition (OCR). This guide demonstrates how to perform single-line OCR using Kalosm's vision module.

## Adding dependencies

Before we get started, we need to add an additional crate for image loading. Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
# Your Kalosm dependency added in the start of the documentation...
image = "0.24.7"
```

## Creating an OCR Model

Kalosm's `vision` module provides functionality for text recognition in images. In this example, the `Ocr::builder()` method is used to create an OCR model that can transcribe single lines of text.

```rust
{{#include src/doc_snippets/text_recognition.rs:create_model}}
```

## Loading Image

Next, we need to load an image that contains text. The `image` crate provides the [open](https://docs.rs/image/latest/image/fn.open.html) method to load an image from a file path, or the [Reader](https://docs.rs/image/latest/image/io/struct.Reader.html) for more advanced loading options.

```rust
{{#include src/doc_snippets/text_recognition.rs:create_model}}
```

Replace the file path with the location of your image. This loaded image will be processed for text recognition.

## Recognizing Text

Finally, we can use the `recognize_text` method to extract text information from the image. The `recognize_text` method takes an `OcrInferenceSettings` struct as input. This struct contains the image to be processed, as well as other settings that can be used to customize the OCR process.

```rust
{{#include src/doc_snippets/text_recognition.rs:recognize_text}}
```

## Conclusion

This example provides a basic structure for performing single-line OCR using Kalosm's vision module. You can combine text recognition with an [LLM](./llms/index.md) to analyze complex documents or photos.
