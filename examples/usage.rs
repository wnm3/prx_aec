/*
git submodule update --init
cargo run --example usage
*/

fn main() {
    let sample_rate = 16000; // 16kHz
    let filter_length = (sample_rate as f32 * 0.1).round() as i32; // 0.1s
    let mut aec = aec_rs::Aec::new(sample_rate, filter_length);
    aec.set_sample_rate(sample_rate);

    let frame_size = aec.get_frame_size();

    let mut input_frame: Vec<i16> = vec![0; frame_size as usize];
    let mut reference_frame: Vec<i16> = vec![0; frame_size as usize];

    for i in 0..frame_size as usize {
        input_frame[i] = (i % 10) as i16; // Sample data
        reference_frame[i] = (i % 5) as i16; // Simulated echo
    }

    // Process the frames with the AEC system
    let cancelled = aec.cancel(&mut input_frame, &mut reference_frame);
    println!("{}", cancelled.len());
}
