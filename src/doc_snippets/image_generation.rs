#[tokio::main]
async fn main() {
    // ANCHOR: create_model
    use kalosm::vision::*;
    let model = Wuerstchen::builder().build().await.unwrap();
    // ANCHOR_END: create_model
    // ANCHOR: create_settings
    let settings = WuerstchenInferenceSettings::new(
        "a cute cat with a hat in a room covered with fur with incredible detail",
    )
    .with_denoiser_steps(2);
    // ANCHOR_END: create_settings
    // ANCHOR: run_model
    let mut images = model.run(settings);
    let mut index = 0;
    while let Some(img) = images.next().await {
        img.as_ref().save(&format!("{}.png", index)).unwrap();
        index += 1;
    }
    // ANCHOR_END: run_model
}
