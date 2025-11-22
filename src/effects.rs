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
