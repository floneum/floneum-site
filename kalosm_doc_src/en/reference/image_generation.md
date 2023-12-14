# Image Generation

Kalosm supports image generation from text descriptions. This guide demonstrates the basic steps for performing image generation using Kalosm's vision module.

## Kalosm's Image Generation Module

Kalosm's vision module includes models for image generation. In this example, the `Wuerstchen::builder()` method is used to create an image generation model. You can pass options to the `builder` method to customize the model.

```rust
{{#include src/doc_snippets/image_generation.rs:create_model}}
```

## Setting Generation Parameters

Next, we need to set the generation parameters. The `WuerstchenInferenceSettings` struct is used to create the generation parameters. We will set the prompt and number of steps for the generation process. The more steps you use, the more detailed the generated image will be (up to the resolution of the output image).

```rust
{{#include src/doc_snippets/image_generation.rs:create_settings}}
```


## Performing Image Generation

Finally, we can use the `run` method to generate images based on the given description. The `run` method takes the `WuerstchenInferenceSettings` struct we created as input and returns a list of generated images. Then we can iterate through the generated images and save them to the file system.

```rust
{{#include src/doc_snippets/image_generation.rs:run_model}}
```

## Conclusion

This guide demonstrates the basic steps for performing image generation using Kalosm's vision module with the Wuerstchen model. You can combine image generation with an [LLM](./llms/index.md) to generate images based on descriptions that the LLM generates to automatically generate reliant images for a situation or automatically improve prompts.
