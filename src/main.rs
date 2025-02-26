use clap::Parser;
use csv::ReaderBuilder;
use std::fs::{self, File};
use std::io::Write;

#[derive(Parser)]
struct Cli {
    /// CSV file to be converted to Markdown table
    #[clap(short, long)]
    input: String,

    /// Optional file to prepend to the resulting markdown file
    #[clap(short, long)]
    prepend: Option<String>,

    /// Output markdown file
    #[clap(short, long)]
    output: String,
}

fn csv_to_markdown_table(input_path: &str) -> Result<String, anyhow::Error> {
    let mut rdr = ReaderBuilder::new().from_path(input_path)?;
    let headers = rdr.headers()?.clone();
    let mut table = String::new();

    // Write header row
    for (i, header) in headers.iter().enumerate() {
        if i > 0 {
            table.push('|');
        }
        table.push_str(header);
    }
    table.push('\n');

    // Write separator row
    for i in 0..headers.len() {
        if i > 0 {
            table.push('|');
        }
        table.push_str("---");
    }
    table.push('\n');

    // Write data rows
    for result in rdr.records() {
        let record = result?;
        for (i, field) in record.iter().enumerate() {
            if i > 0 {
                table.push('|');
            }
            table.push_str(field);
        }
        table.push('\n');
    }

    Ok(table)
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    // Convert CSV to Markdown table
    let mut markdown_content = csv_to_markdown_table(&cli.input)?;

    // Optionally prepend another file's content
    if let Some(prepend_path) = &cli.prepend {
        let prepend_content = fs::read_to_string(prepend_path)?;
        markdown_content.insert_str(0, &prepend_content);
    }

    // Write the Markdown table to the output file
    let mut output_file = File::create(&cli.output)?;
    write!(output_file, "{}", markdown_content)?;

    Ok(())
}

#[cfg(test)]
mod tests;
