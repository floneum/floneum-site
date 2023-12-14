fn main() {
    // ANCHOR: create_model
    use kalosm::vision::*;
    let model = Wuerstchen::builder().build().unwrap();
    // ANCHOR_END: create_model
    // ANCHOR: create_settings
    let settings = WuerstchenInferenceSettings::new(
        "a cute cat with a hat in a room covered with fur with incredible detail",
    )
    .with_n_steps(2);
    // ANCHOR_END: create_settings
    // ANCHOR: run_model
    let images = model.run(settings).unwrap();
    for (i, img) in images.iter().enumerate() {
        img.save(&format!("{}.png", i)).unwrap();
    }
    // ANCHOR_END: run_model
}
