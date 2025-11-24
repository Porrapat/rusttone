#![allow(non_snake_case)]
#![allow(unused)]

use hound::{WavReader, WavWriter};

pub fn apply_echo(input: &std::path::Path, output: &std::path::Path) {
    // 1. Read WAV
    let mut reader = WavReader::open(input).expect("Failed to open WAV");
    let spec = reader.spec();

    // 2. Convert sample → f32
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32)
        .collect();

    // 3. Sample Setting (We will make UI later)
    let delay = 5000 as usize;
    let a: f32 = 0.6;

    // 4. Use old algorithm!
    let processed_f32 = single_echo(&samples, delay, a);

    // 5. Convert back to i16
    let processed_i16: Vec<i16> = processed_f32
        .iter()
        .map(|x| x.clamp(-32768.0, 32767.0) as i16)
        .collect();

    // 6. Write WAV
    let mut writer = WavWriter::create(output, spec).unwrap();
    for s in processed_i16 {
        writer.write_sample(s).unwrap();
    }
}

pub fn apply_multi(input: &std::path::Path, output: &std::path::Path) {
    let mut reader = WavReader::open(input).unwrap();
    let spec = reader.spec();

    // 1. Read WAV
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32)
        .collect();

    // 3. Sample Setting (We will make UI later)
    let delay = 6000 as usize;
    let a: f32 = 0.4;
    let n_echo = 5;

    // 4. Use old algorithm!
    let processed = multiple_echo(&samples, delay, a, n_echo);

    // Convert back
    let processed_i16: Vec<i16> = processed
        .iter()
        .map(|x| x.clamp(-32768.0, 32767.0) as i16)
        .collect();

    let mut writer = WavWriter::create(output, spec).unwrap();
    for s in processed_i16 {
        writer.write_sample(s).unwrap();
    }
}


pub fn single_echo(samples: &Vec<f32>, delay: usize, a: f32) -> Vec<f32> {
    let mut out = vec![0.0; samples.len()];

    for i in 0..samples.len() {
        out[i] = samples[i];
        if i >= delay {
            out[i] += a * samples[i - delay];
        }
    }

    out
}

pub fn multiple_echo(samples: &Vec<f32>, delay: usize, a: f32, n_echo: usize) -> Vec<f32> {
    let mut out = vec![0.0; samples.len()];

    for i in 0..samples.len() {
        out[i] += samples[i];

        for k in 1..=n_echo {
            let d = k * delay;
            if i >= d {
                out[i] += (k as f32) * a * samples[i - d];
            }
        }
    }

    out
}

pub fn reverb(
    samples: &Vec<f32>,
    R: &[usize],   // delays
    a: &[f32],     // gains
) -> Vec<f32> {
    // --- Comb filters (IIR echo) 4 Sets ---
    let mut d1 = vec![0.0; samples.len()];
    let mut d2 = vec![0.0; samples.len()];
    let mut d3 = vec![0.0; samples.len()];
    let mut d4 = vec![0.0; samples.len()];

    for i in 0..samples.len() {
        // d1
        d1[i] = samples[i];
        if i >= R[0] {
            d1[i] += a[0] * d1[i - R[0]];
        }

        // d2
        d2[i] = samples[i];
        if i >= R[1] {
            d2[i] += a[1] * d2[i - R[1]];
        }

        // d3
        d3[i] = samples[i];
        if i >= R[2] {
            d3[i] += a[2] * d3[i - R[2]];
        }

        // d4
        d4[i] = samples[i];
        if i >= R[3] {
            d4[i] += a[3] * d4[i - R[3]];
        }
    }

    let mut d_iir = vec![0.0; samples.len()];
    for i in 0..samples.len() {
        d_iir[i] = d1[i] + d2[i] + d3[i] + d4[i];
    }

    // --- All-pass 1 ---
    let mut ap1 = vec![0.0; samples.len()];
    for i in 0..samples.len() {
        ap1[i] = d_iir[i];
        if i >= R[4] {
            ap1[i] += -a[4] * d_iir[i] + a[4] * ap1[i - R[4]] + d_iir[i - R[4]];
        }
    }

    // --- All-pass 2 ---
    let mut ap2 = vec![0.0; samples.len()];
    for i in 0..samples.len() {
        ap2[i] = ap1[i];
        if i >= R[5] {
            ap2[i] += -a[5] * ap1[i] + a[5] * ap2[i - R[5]] + ap1[i - R[5]];
        }
    }

    // --- Final output ---
    let mut out = vec![0.0; samples.len()];
    for i in 0..samples.len() {
        out[i] = samples[i] + a[6] * ap2[i]; // dry + wet
    }

    out
}



// ==========================================
//  REVERB ENGINE (ใหม่ทั้งหมด)
// ==========================================

// all-pass filter ตามสูตรเดิม (ถ้าไม่มีให้เพิ่มอันนี้แทน)
pub fn alpas_filter(samples: &Vec<f32>, delay: usize, a: f32) -> Vec<f32> {
    let mut out = vec![0.0; samples.len()];

    for i in 0..samples.len() {
        if i >= delay {
            out[i] = -a * samples[i] + samples[i - delay] + a * out[i - delay];
        } else {
            out[i] = samples[i];
        }
    }
    out
}

// multi-echo generator แบบ feedback
pub fn multi_echo_iir(samples: &Vec<f32>, delay: usize, a: f32) -> Vec<f32> {
    let mut out = samples.clone();

    for i in delay..samples.len() {
        out[i] += a * out[i - delay];
    }

    out
}

// ==========================================
//  REVERB CORE (สูตรแทน apply_reverb_core เดิม)
// ==========================================
pub fn apply_reverb_core(
    samples: &Vec<f32>,
    R: &[usize; 6],
    a: &[f32; 7],
) -> Vec<f32> {

    // 1) Early reflections (IIR echoes)
    let d1 = multi_echo_iir(samples, R[0], a[0]);
    let d2 = multi_echo_iir(samples, R[1], a[1]);
    let d3 = multi_echo_iir(samples, R[2], a[2]);
    let d4 = multi_echo_iir(samples, R[3], a[3]);

    // รวม 4 ตัวแรก
    let mut sum = vec![0.0; samples.len()];
    for i in 0..samples.len() {
        sum[i] = d1[i] + d2[i] + d3[i] + d4[i];
    }

    // 2) All-pass filters 2 ชั้น
    let ap1 = alpas_filter(&sum, R[4], a[4]);
    let ap2 = alpas_filter(&ap1, R[5], a[5]);

    // 3) Mix dry/wet final
    let wet = a[6];
    let mut out = vec![0.0; samples.len()];

    for i in 0..samples.len() {
        out[i] = samples[i] + wet * ap2[i];
    }

    out
}

pub fn apply_reverb(input: &std::path::Path, output: &std::path::Path) {
    let mut reader = WavReader::open(input).unwrap();
    let spec = reader.spec();

    // WAV → f32
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32)
        .collect();

    // Delay & gain parameters  (6 delay, 7 gain)
    let R = [
        spec.sample_rate as usize / 7,
        spec.sample_rate as usize / 6,
        spec.sample_rate as usize / 5,
        spec.sample_rate as usize / 4,
        spec.sample_rate as usize / 3,
        spec.sample_rate as usize / 2,
    ];

    let a = [0.6, 0.5, 0.4, 0.3, 0.7, 0.7, 0.5];

    // คำนวณ Reverb
    let processed = apply_reverb_core(&samples, &R, &a);

    // f32 → i16
    let processed_i16: Vec<i16> = processed
        .iter()
        .map(|v| v.clamp(-32768.0, 32767.0) as i16)
        .collect();

    // เขียนไฟล์กลับ WAV
    let mut writer = WavWriter::create(output, spec).unwrap();
    for s in processed_i16 {
        writer.write_sample(s).unwrap();
    }
}
