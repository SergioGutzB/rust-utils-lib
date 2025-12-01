use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

/// Read the contents of a file and return it as a string.
///
/// # Errors
///
/// Returns an error if the file doesn't exist, can't be read, or contains invalid UTF-8.
///
/// # Examples
///
/// ```no_run
/// use rust_utils_lib::read_file;
///
/// match read_file("example.txt") {
///     Ok(contents) => println!("File contents: {}", contents),
///     Err(e) => eprintln!("Error reading file: {}", e),
/// }
/// ```
pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Write a string to a file, creating the file if it doesn't exist or overwriting it if it does.
///
/// # Errors
///
/// Returns an error if the file cannot be created or written to.
///
/// # Examples
///
/// ```no_run
/// use rust_utils_lib::write_file;
///
/// let content = "Hello, World!";
/// match write_file("output.txt", content) {
///     Ok(_) => println!("File written successfully"),
///     Err(e) => eprintln!("Error writing file: {}", e),
/// }
/// ```
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Append content to an existing file, creating it if it doesn't exist.
///
/// # Errors
///
/// Returns an error if the file cannot be opened or written to.
///
/// # Examples
///
/// ```no_run
/// use rust_utils_lib::append_to_file;
///
/// match append_to_file("log.txt", "New log entry\n") {
///     Ok(_) => println!("Content appended successfully"),
///     Err(e) => eprintln!("Error appending to file: {}", e),
/// }
/// ```
pub fn append_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper function to clean up test files
    fn cleanup_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_write_and_read_file() {
        let path = "test_write_read.txt";
        let content = "Hello, Rust!";

        // Write to file
        let write_result = write_file(path, content);
        assert!(write_result.is_ok());

        // Read from file
        let read_result = read_file(path);
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);

        cleanup_file(path);
    }

    #[test]
    fn test_write_overwrites_existing_file() {
        let path = "test_overwrite.txt";
        let initial_content = "Initial content";
        let new_content = "New content";

        // Write initial content
        write_file(path, initial_content).unwrap();

        // Overwrite with new content
        write_file(path, new_content).unwrap();

        // Read and verify new content
        let result = read_file(path).unwrap();
        assert_eq!(result, new_content);

        cleanup_file(path);
    }

    #[test]
    fn test_append_to_new_file() {
        let path = "test_append_new.txt";
        let content = "First line\n";

        // Clean up if file exists
        cleanup_file(path);

        // Append to non-existent file (should create it)
        let result = append_to_file(path, content);
        assert!(result.is_ok());

        // Verify content
        let read_result = read_file(path).unwrap();
        assert_eq!(read_result, content);

        cleanup_file(path);
    }

    #[test]
    fn test_append_to_existing_file() {
        let path = "test_append_existing.txt";
        let initial_content = "Line 1\n";
        let appended_content = "Line 2\n";

        // Create file with initial content
        write_file(path, initial_content).unwrap();

        // Append new content
        append_to_file(path, appended_content).unwrap();

        // Verify both contents are present
        let result = read_file(path).unwrap();
        assert_eq!(result, format!("{}{}", initial_content, appended_content));

        cleanup_file(path);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let path = "nonexistent_file_12345.txt";

        // Ensure file doesn't exist
        cleanup_file(path);

        // Try to read non-existent file
        let result = read_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_empty_string() {
        let path = "test_empty.txt";
        let content = "";

        write_file(path, content).unwrap();
        let result = read_file(path).unwrap();
        assert_eq!(result, content);

        cleanup_file(path);
    }

    #[test]
    fn test_multiple_appends() {
        let path = "test_multiple_appends.txt";

        // Clean up if exists
        cleanup_file(path);

        // Multiple appends
        append_to_file(path, "Line 1\n").unwrap();
        append_to_file(path, "Line 2\n").unwrap();
        append_to_file(path, "Line 3\n").unwrap();

        // Verify all lines are present
        let result = read_file(path).unwrap();
        assert_eq!(result, "Line 1\nLine 2\nLine 3\n");

        cleanup_file(path);
    }

    #[test]
    fn test_unicode_content() {
        let path = "test_unicode.txt";
        let content = "Hello 👋 世界 café";

        write_file(path, content).unwrap();
        let result = read_file(path).unwrap();
        assert_eq!(result, content);

        cleanup_file(path);
    }

    #[test]
    fn test_multiline_content() {
        let path = "test_multiline.txt";
        let content = "Line 1\nLine 2\nLine 3\n";

        write_file(path, content).unwrap();
        let result = read_file(path).unwrap();
        assert_eq!(result, content);

        cleanup_file(path);
    }
}
