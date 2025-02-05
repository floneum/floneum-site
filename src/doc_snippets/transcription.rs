#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // ANCHOR: create_model
    use kalosm::sound::*;

    // Create a new whisper model
    let model = Whisper::new().await?;
    // ANCHOR_END: create_model

    // ANCHOR: create_model_with_loading_handler
    let model = Whisper::builder()
        .build_with_loading_handler(|progress| match &progress {
            ModelLoadingProgress::Downloading {
                source,
                progress: file_loading_progress,
            } => {
                let progress = (progress.progress() * 100.0) as u32;
                let elapsed = file_loading_progress.start_time.elapsed().as_secs_f32();
                println!("Downloading file {source} {progress}% ({elapsed}s)");
            }
            ModelLoadingProgress::Loading { progress } => {
                let progress = (progress * 100.0) as u32;
                println!("Loading model {progress}%");
            }
        })
        .await?;
    // ANCHOR_END: create_model_with_loading_handler

    // ANCHOR: record_audio
    // Stream audio from the microphone
    let mic = MicInput::default();
    let stream = mic.stream();
    // ANCHOR_END: record_audio

    // ANCHOR: transcribe
    // Transcribe the audio.
    let mut transcribed = stream.transcribe(model);

    // As the model transcribes the audio, print the text to the console.
    transcribed.to_std_out().await?;
    // ANCHOR_END: transcribe

    Ok(())
}
