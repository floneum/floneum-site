# Audio Transcription

Audio transcription with Kalosm involves the use of Automatic Speech Recognition (ASR) models to convert spoken language into written text. This process is crucial in applications ranging from voice assistants to transcription services. Let's explore the key concepts behind audio transcription using Kalosm:

## 1. Automatic Speech Recognition (ASR)

ASR is a technology that converts spoken language into written text. It analyzes audio signals to identify and transcribe spoken words.

## 2. Whisper ASR Model

Kalosm provides the Whisper ASR model, designed for accurate and efficient speech recognition. Developers can choose different Whisper models based on their requirements, such as `DistilLargeV2` in the example.

## 3. Audio Input

The audio source can be diverse, ranging from pre-recorded files to real-time input from a microphone. The example code uses `kalosm_sound::MicInput::record_until` to record audio from the microphone for a specified duration.

## 4. Transcription Process

The recorded audio is then passed through the Whisper ASR model for transcription. Kalosm's `transcribe` method is utilized to perform this operation. The output is a stream of transcribed segments.

## 5. Real-time Output

The transcribed segments are processed in real-time. The code snippet prints the transcribed text along with corresponding time stamps, indicating when each segment started and ended.

## 6. Handling No Speech

The code includes a check for the probability of no speech in a given segment. If the probability is high (above 0.90 in the example), it indicates that no speech was detected during that period.

## 7. Iterative Processing

The entire process is typically performed iteratively in a loop, allowing continuous transcription of audio segments.

## 8. Application Scenarios

Audio transcription with Kalosm finds applications in voice-controlled systems, transcription services, virtual assistants, and more. Developers can tailor the use of ASR to specific contexts and requirements.

# Example: Audio Transcription with Kalosm API

```rust
// Importing dependencies...

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create a new small whisper model.
    let model = WhisperBuilder::default()
        .with_source(WhisperSource::DistilLargeV2)
        .build()?;

    let mut current_time_stamp = 0.0;
    loop {
        // Record audio from the microphone for 5 seconds.
        let audio = kalosm_sound::MicInput::default()
            .record_until(Instant::now() + Duration::from_secs(5))
            .await?;

        // Transcribe the audio.
        let mut transcribed = model.transcribe(audio)?;

        // As the model transcribes the audio, print the text to the console.
        while let Some(transcribed) = transcribed.next().await {
            let start = current_time_stamp + transcribed.start();
            let end = start + transcribed.duration();
            if transcribed.probability_of_no_speech() < 0.90 {
                let text = transcribed.text();
                println!("({:01} - {:01}): {}", start, end, text);
            } else {
                println!(
                    "({:01} - {:01}): <no speech> ({})",
                    start,
                    end,
                    transcribed.text()
                );
            }
            current_time_stamp = end;
        }
    }
}
```

## Explanation of API Usage:

1. **Whisper ASR Model Creation:**

   Create a Whisper ASR model using `WhisperBuilder`. In this example, we use the `DistilLargeV2` source for the model.

2. **Audio Recording:**

   Record audio from the microphone for 5 seconds using `MicInput::record_until`.

3. **Transcribing Audio:**

   Transcribe the recorded audio using the Whisper ASR model. The result is a stream of transcribed segments.

4. **Real-time Output Processing:**

   Iterate over the transcribed segments in real-time, printing the transcribed text along with corresponding time stamps.

5. **Handling No Speech:**

   Check the probability of no speech in a given segment. If the probability is high, indicate that no speech was detected.

This example showcases how to use Kalosm's API to integrate audio transcription capabilities into your Rust application. Developers can adapt and extend this code to suit specific use cases, such as voice-controlled systems or transcription services. Understanding the API calls and their sequence helps developers effectively use Kalosm for audio transcription.