use rust_utils_lib::{append_to_file, read_file, write_file};
use std::fs;

// Helper function to clean up test files
fn cleanup_file(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_write_and_read_integration() {
    let path = "integration_test_write_read.txt";
    let content = "Integration test content";

    // Write file
    write_file(path, content).expect("Failed to write file");

    // Read file
    let read_content = read_file(path).expect("Failed to read file");

    assert_eq!(read_content, content);
    cleanup_file(path);
}

#[test]
fn test_overwrite_behavior() {
    let path = "integration_test_overwrite.txt";

    // Write initial content
    write_file(path, "Original content").unwrap();

    // Overwrite with new content
    write_file(path, "Updated content").unwrap();

    // Verify only new content exists
    let result = read_file(path).unwrap();
    assert_eq!(result, "Updated content");

    cleanup_file(path);
}

#[test]
fn test_append_workflow() {
    let path = "integration_test_append.txt";

    // Clean up first
    cleanup_file(path);

    // Create file by appending
    append_to_file(path, "First entry\n").unwrap();
    append_to_file(path, "Second entry\n").unwrap();
    append_to_file(path, "Third entry\n").unwrap();

    // Read and verify
    let result = read_file(path).unwrap();
    assert_eq!(result, "First entry\nSecond entry\nThird entry\n");

    cleanup_file(path);
}

#[test]
fn test_write_then_append() {
    let path = "integration_test_write_append.txt";

    // Write initial content
    write_file(path, "Header\n").unwrap();

    // Append additional content
    append_to_file(path, "Line 1\n").unwrap();
    append_to_file(path, "Line 2\n").unwrap();

    // Verify all content
    let result = read_file(path).unwrap();
    assert_eq!(result, "Header\nLine 1\nLine 2\n");

    cleanup_file(path);
}

#[test]
fn test_large_file_operations() {
    let path = "integration_test_large.txt";

    // Create large content
    let mut large_content = String::new();
    for i in 0..1000 {
        large_content.push_str(&format!("Line {}\n", i));
    }

    // Write large file
    write_file(path, &large_content).unwrap();

    // Read it back
    let result = read_file(path).unwrap();
    assert_eq!(result, large_content);

    cleanup_file(path);
}

#[test]
fn test_unicode_and_special_characters() {
    let path = "integration_test_unicode.txt";

    let content = "English\nEspañol: café\n日本語: こんにちは\nEmojis: 👋🌍🚀\n";

    write_file(path, content).unwrap();
    let result = read_file(path).unwrap();
    assert_eq!(result, content);

    cleanup_file(path);
}

#[test]
fn test_error_handling_nonexistent_file() {
    let path = "this_file_should_not_exist_12345.txt";

    // Ensure it doesn't exist
    cleanup_file(path);

    // Try to read non-existent file
    let result = read_file(path);
    assert!(result.is_err());
}

#[test]
fn test_empty_file_operations() {
    let path = "integration_test_empty.txt";

    // Write empty file
    write_file(path, "").unwrap();

    // Read empty file
    let result = read_file(path).unwrap();
    assert_eq!(result, "");

    // Append to empty file
    append_to_file(path, "Now not empty").unwrap();

    // Verify
    let result = read_file(path).unwrap();
    assert_eq!(result, "Now not empty");

    cleanup_file(path);
}

#[test]
fn test_file_with_different_line_endings() {
    let path = "integration_test_line_endings.txt";

    // Unix line endings
    let content = "Line 1\nLine 2\nLine 3\n";

    write_file(path, content).unwrap();
    let result = read_file(path).unwrap();
    assert_eq!(result, content);

    cleanup_file(path);
}

#[test]
fn test_sequential_operations() {
    let path = "integration_test_sequential.txt";

    // Write
    write_file(path, "Start\n").unwrap();

    // Read
    let content1 = read_file(path).unwrap();
    assert_eq!(content1, "Start\n");

    // Append
    append_to_file(path, "Middle\n").unwrap();

    // Read again
    let content2 = read_file(path).unwrap();
    assert_eq!(content2, "Start\nMiddle\n");

    // Overwrite
    write_file(path, "End\n").unwrap();

    // Final read
    let content3 = read_file(path).unwrap();
    assert_eq!(content3, "End\n");

    cleanup_file(path);
}

#[test]
fn test_create_file_in_current_directory() {
    let path = "simple_test.txt";

    cleanup_file(path);

    // Create and write
    write_file(path, "Simple test").unwrap();

    // Verify it exists and has content
    assert!(fs::metadata(path).is_ok());

    let content = read_file(path).unwrap();
    assert_eq!(content, "Simple test");

    cleanup_file(path);
}
