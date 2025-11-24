use clap::{Parser, Subcommand};
use hound::*;

mod effects;

/// RustTone command-line tool
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Single echo: y[n] = x[n] + a*x[n-R]
    Single {
        input: String,
        output: String,
        delay: usize,
        a: f32,
    },

    /// Multiple echo: y[n] = x[n] + a*x[n-R] + 2a*x[n-2R] + ...
    Multi {
        input: String,
        output: String,
        delay: usize,
        a: f32,
        echoes: usize,
    },

    /// Reverb
    Reverb {
        input: String,
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Single {
            input,
            output,
            delay,
            a,
        } => {
            println!("Applying SINGLE echo...");
            process_single(&input, &output, delay, a);
        }

        Commands::Multi {
            input,
            output,
            delay,
            a,
            echoes,
        } => {
            println!("Applying MULTIPLE echo...");
            process_multi(&input, &output, delay, a, echoes);
        }

        Commands::Reverb { input, output } => {
            println!("Applying Reverb echo...");
            process_reverb(&input, &output);
        }
    }
}

fn process_single(input: &str, output: &str, delay: usize, a: f32) {
    let (spec, samples) = read_wav(input);
    let processed = effects::single_echo(&samples, delay, a);
    write_wav(output, spec, &processed);
}

fn process_multi(input: &str, output: &str, delay: usize, a: f32, echoes: usize) {
    let (spec, samples) = read_wav(input);
    let processed = effects::multiple_echo(&samples, delay, a, echoes);
    write_wav(output, spec, &processed);
}

fn process_reverb(input: &str, output: &str) {
    let (spec, samples) = read_wav(input);

    // Value r and a, just in basic
    let r = vec![700, 900, 600, 400, 450, 390];
    let a = vec![0.6, 0.4, 0.2, 0.1, 0.7, 0.6, 0.8];

    let processed = effects::reverb(&samples, &r, &a);

    write_wav(output, spec, &processed);
}

/// Read WAV -> Vec<f32>
fn read_wav(path: &str) -> (WavSpec, Vec<f32>) {
    let mut reader = WavReader::open(path).expect("cannot open input WAV");
    let spec = reader.spec();

    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|x| x.unwrap() as f32 / i16::MAX as f32)
        .collect();

    (spec, samples)
}

/// Write Vec<f32> -> WAV file
fn write_wav(path: &str, spec: WavSpec, samples: &[f32]) {
    let mut writer = WavWriter::create(path, spec).expect("cannot write output WAV");

    for s in samples {
        let v = (s * i16::MAX as f32) as i16;
        writer.write_sample(v).unwrap();
    }

    println!("Saved {}", path);
}
