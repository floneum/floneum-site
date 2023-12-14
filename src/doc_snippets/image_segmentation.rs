fn main() {
    // ANCHOR: create_model
    use kalosm::vision::*;
    let model = SegmentAnything::builder().build().unwrap();
    // ANCHOR_END: create_model
    // ANCHOR: load_image
    let image = image::open("examples/landscape.jpg").unwrap();
    // ANCHOR_END: load_image
    // ANCHOR: create_settings
    let x = image.width() / 2;
    let y = image.height() / 2;
    let settings = SegmentAnythingInferenceSettings::new(image)
        .unwrap()
        .add_goal_point(x, y);
    // ANCHOR_END: create_settings
    // ANCHOR: run_model
    let images = model.segment_from_points(settings).unwrap();

    images.save("out.png").unwrap();
    // ANCHOR_END: run_model
}
