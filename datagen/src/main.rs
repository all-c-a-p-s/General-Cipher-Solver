use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

pub mod ciphers;
pub mod datagen;
pub mod features;

fn write_to_csv(data: &[(String, [f32; 47])], filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    write!(writer, "label")?;
    for i in 0..47 {
        write!(writer, ",feature_{}", i)?;
    }
    writeln!(writer)?;

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
    let generated = datagen::generate_once();
    write_to_csv(&generated.as_flattened(), "test.csv")?;
    Ok(())
}
