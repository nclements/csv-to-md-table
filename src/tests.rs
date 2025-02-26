use std::fs;
use std::io::{self, Write};
use tempfile::NamedTempFile;

#[test]
fn test_csv_to_markdown_table() -> io::Result<()> {
    // Create a temporary CSV file with sample data
    let mut csv_file = NamedTempFile::new()?;
    writeln!(csv_file, "Name,Age,City")?;
    writeln!(csv_file, "Alice,30,New York")?;
    writeln!(csv_file, "Bob,25,Los Angeles")?;

    // Convert CSV to Markdown table
    let markdown_table = super::csv_to_markdown_table(csv_file.path().to_str().unwrap()).unwrap();

    // Expected Markdown table content
    let expected_table =
        String::from("Name|Age|City\n---|---|---\nAlice|30|New York\nBob|25|Los Angeles\n");

    assert_eq!(markdown_table, expected_table);
    Ok(())
}

#[test]
fn test_csv_to_markdown_with_prepend() -> io::Result<()> {
    // Create a temporary CSV file with sample data
    let mut csv_file = NamedTempFile::new()?;
    writeln!(csv_file, "Name,Age,City")?;
    writeln!(csv_file, "Alice,30,New York")?;
    writeln!(csv_file, "Bob,25,Los Angeles")?;

    // Create a temporary prepend file with sample data
    let mut prepend_file = NamedTempFile::new()?;
    writeln!(prepend_file, "# Sample Markdown\nThis is a test.")?;

    // Convert CSV to Markdown table and prepend content
    let markdown_table = super::csv_to_markdown_table(csv_file.path().to_str().unwrap()).unwrap();
    let mut prepend_content = fs::read_to_string(prepend_file.path()).unwrap();
    prepend_content.push_str(&markdown_table);

    // Expected Markdown table content with prepended data
    let expected_table = String::from(
            "# Sample Markdown\nThis is a test.\nName|Age|City\n---|---|---\nAlice|30|New York\nBob|25|Los Angeles\n",
        );
    assert_eq!(prepend_content, expected_table);
    Ok(())
}
