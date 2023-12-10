# Optical Character Recognition

Single-Line OCR (Optical Character Recognition) using Kalosm allows developers to extract text information from images. The example provided demonstrates the fundamental steps to perform single-line OCR using Kalosm's vision module.

## Kalosm's OCR Module

Kalosm's OCR module provides functionality for text recognition in images. In this example, the `Ocr::builder()` method is used to create an OCR model.

```rust
let mut model = Ocr::builder().build().unwrap();
```

This line initializes an OCR model that will be used to recognize text in images.

## Loading Image

The example uses the `image::open` function to load an image from the file system.

```rust
let image = image::open("examples/ocr.png").unwrap();
```

Replace the file path with the location of your image. This loaded image will be processed for text recognition.

## Text Recognition

The OCR model is then used to recognize text in the loaded image. The `recognize_text` method takes an `OcrInferenceSettings` object, which is constructed using the loaded image.

```rust
let text = model
    .recognize_text(OcrInferenceSettings::new(image).unwrap())
    .unwrap();
```

The `recognize_text` method processes the image and extracts the text information.

## Displaying Results

The recognized text is printed to the console.

```rust
println!("{}", text);
```

This line outputs the recognized text to the console, allowing developers to view the OCR results.

## Conclusion

This example provides a basic structure for performing single-line OCR using Kalosm's vision module. Developers can customize this code to suit their specific use cases, handle errors, and integrate OCR functionality into their applications. Understanding the steps involved in single-line OCR with Kalosm enables developers to efficiently extract textual information from images for various applications.