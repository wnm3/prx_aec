/*
wget https://github.com/thewh1teagle/aec-rs/releases/download/audio-files/rec.wav
wget https://github.com/thewh1teagle/aec-rs/releases/download/audio-files/echo.wav
wget https://github.com/thewh1teagle/aec-rs/releases/download/audio-files/voice.wav
cargo run --example wav rec.wav echo.wav cancelled.wav
*/

fn main() {
    // 16kHz mono int16 same length
    let rec_path = std::env::args().nth(1).expect("Please specify echo path");
    let echo_path = std::env::args().nth(2).expect("Please specify rec path");
    let out_path = std::env::args().nth(3).expect("Please specify out path");

    // Read echo samples
    let mut reader = hound::WavReader::open(echo_path).unwrap();
    let echo_samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    // Read recorded samples
    let mut reader = hound::WavReader::open(rec_path).unwrap();
    let rec_samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    let sample_rate = 16000; // 16kHz
    let filter_length = (sample_rate as f32 * 0.1).round() as i32; // 0.1s
    let mut aec = aec_rs::Aec::new(sample_rate, filter_length);
    aec.set_sample_rate(sample_rate);

    let frame_size = aec.get_frame_size();

    // Prepare output WAV writer
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&out_path, spec).unwrap();

    // Process samples frame by frame
    let mut output_samples = Vec::new();
    let mut input_frame = vec![0; frame_size as usize];
    let mut reference_frame = vec![0; frame_size as usize];

    for i in (0..rec_samples.len()).step_by(frame_size as usize) {
        let end = usize::min(i + frame_size as usize, rec_samples.len());

        // Fill frames
        input_frame[..end - i].copy_from_slice(&rec_samples[i..end]);
        reference_frame[..end - i].copy_from_slice(&echo_samples[i..end]);

        // Apply AEC cancellation
        let cancelled = aec.cancel(&mut input_frame, &mut reference_frame);
        output_samples.extend_from_slice(&cancelled);

        // Write to output WAV file
        for &sample in &cancelled {
            writer.write_sample(sample).unwrap();
        }
    }

    println!("Output written to {}", out_path);
}
