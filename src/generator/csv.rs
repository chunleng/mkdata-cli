use std::path::Path;

use anyhow::Result;
use csv::WriterBuilder;
use fake::{faker::lorem::raw::Word, locales::EN, Fake};
use std::fs::File;

pub fn generate_csv(path: Box<Path>, rows: usize, cols: Vec<String>) -> Result<()> {
    let mut wtr = WriterBuilder::new()
        .delimiter(b',')
        .quote_style(csv::QuoteStyle::Necessary)
        .from_writer(File::create(&path)?);

    // Write headers
    wtr.write_record(&cols)?;

    // Write rows
    for _ in 0..rows {
        let mut row: Vec<String> = Vec::new();
        for _ in &cols {
            row.push(Word(EN).fake())
        }
        wtr.write_record(&row)?;
    }

    wtr.flush()?;

    Ok(())
}
