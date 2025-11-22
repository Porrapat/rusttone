use hound::*;

mod effects;

fn main() {
    let input = "./source_wav/namtar_1.wav";
    let output = "./source_wav/namtar_1_out.wav";

    // Read WAV
    let mut reader = WavReader::open(input).expect("Cannot open WAV");
    let spec = reader.spec();
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    // ==== APPLY EFFECT HERE ====
    // Single Echo
    let processed = effects::single_echo(&samples, 8000, 0.5);

    // // Multiple Echo
    // let processed = effects::multiple_echo(&samples, 8000, 0.5, 3);

    // Write output WAV
    let mut writer = WavWriter::create(output, spec).unwrap();
    for s in processed {
        let v = (s * i16::MAX as f32) as i16;
        writer.write_sample(v).unwrap();
    }

    println!("Saved {}", output);
}
