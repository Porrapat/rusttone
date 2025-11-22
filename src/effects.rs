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
    // --- Comb filters (IIR echo) 4 ตัว ---
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
