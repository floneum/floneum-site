# Image Segmentation

Kalosm supports image segmentation with the `SegmentAnything` model. This guide demonstrates the basic steps for performing image segmentation using Kalosm's vision module.

## Kalosm's Image Segmentation Module

Kalosm's vision module provides functionality for image segmentation. In this example, the `SegmentAnything::builder()` method is used to create an image segmentation model. The builder method lets you set options to customize the model, but in this example, we will use the default options.

```rust
{{#include src/doc_snippets/image_segmentation.rs:create_model}}
```

## Loading Image

Next, we need to load an image that contains text. The `image` crate provides the [open](https://docs.rs/image/latest/image/fn.open.html) method to load an image from a file path, or the [Reader](https://docs.rs/image/latest/image/io/struct.Reader.html) for more advanced loading options.

```rust
{{#include src/doc_snippets/image_segmentation.rs:load_image}}
```

Replace the file path with the location of your image. The loaded image will be segmented to extract a region around a point of interest.

## Creating Segmentation Settings

Next we need to create the settings for the image. After adding the image, we can set the goal point for segmentation. The goal point is the point of interest in the image. The segmentation model will extract the region around this point automatically. In this example, we will set the goal point to the center of the image.

```rust
{{#include src/doc_snippets/image_segmentation.rs:create_settings}}
```

## Performing Segmentation

Finally, we can use the `segment_from_points` method to perform segmentation on the image. The `segment_from_points` method takes the `SegmentAnythingInferenceSettings` struct we created as input and returns an mask image of the segment the model extracted. Then we can save the mask image to the file system.

```rust
{{#include src/doc_snippets/image_segmentation.rs:run_model}}
```

## Conclusion

This example demonstrates the basic steps for performing image segmentation using Kalosm's vision module.
