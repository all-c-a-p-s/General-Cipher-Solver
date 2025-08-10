use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub mod ciphers;
pub mod datagen;
pub mod features;

const ITERATIONS: u64 = 10_000;

fn write_to_csv(data: &[(String, [f32; 51])], filename: &str) -> Result<(), Box<dyn Error>> {
    let file_exists = Path::new(filename).exists();

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    let mut writer = BufWriter::new(file);

    if !file_exists {
        write!(writer, "label")?;
        for i in 0..51 {
            write!(writer, ",feature_{}", i)?;
        }
        writeln!(writer)?;
    }

    for (label, features) in data {
        write!(writer, "{}", label)?;
        for &feature in features {
            write!(writer, ",{}", feature)?;
        }
        writeln!(writer)?;
    }

    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let pb = ProgressBar::new(ITERATIONS);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} [{eta_precise}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    for _ in 0..ITERATIONS {
        let generated = datagen::generate_once();
        write_to_csv(&generated.as_flattened(), "data/cipher_data.csv")?;
        pb.inc(1);
    }
    Ok(())
}
