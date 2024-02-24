#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // ANCHOR: create_model
    use kalosm::{audio::*, language::*};

    // Create a new whisper model.
    let model = Whisper::default();
    // ANCHOR_END: create_model

    // ANCHOR: record_audio
    use tokio::time::{Duration, Instant};
    // Record audio from the microphone for 5 seconds.
    let audio = MicInput::default()
        .record_until(Instant::now() + Duration::from_secs(5))
        .await
        .unwrap();
    // ANCHOR_END: record_audio

    // ANCHOR: transcribe
    // Transcribe the audio.
    let transcribed = model.transcribe(audio).unwrap();

    // As the model transcribes the audio, print the text to the console.
    transcribed.to_std_out().await.unwrap();
    // ANCHOR_END: transcribe

    Ok(())
}
