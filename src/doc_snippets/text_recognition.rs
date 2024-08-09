#[tokio::main]
async fn main() {
    // ANCHOR: create_model
    use kalosm::vision::*;
    let mut model = Ocr::builder().build().await.unwrap();
    // ANCHOR_END: create_model
    // ANCHOR: load_image
    let image = image::open("examples/ocr.png").unwrap();
    // ANCHOR_END: load_image
    // ANCHOR: recognize_text
    let text = model
        .recognize_text(OcrInferenceSettings::new(image).unwrap())
        .unwrap();

    println!("{}", text);
    // ANCHOR_END: recognize_text
}
