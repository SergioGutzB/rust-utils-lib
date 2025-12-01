use rust_utils_lib::{count_char, is_palindrome, reverse_string};

#[test]
fn test_is_palindrome_integration() {
    // Simple palindromes
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("noon"));

    // Case-insensitive palindromes
    assert!(is_palindrome("RaceCar"));
    assert!(is_palindrome("Madam"));

    // Palindromes with spaces and punctuation
    assert!(is_palindrome("A man a plan a canal Panama"));
    assert!(is_palindrome("Was it a car or a cat I saw"));

    // Not palindromes
    assert!(!is_palindrome("hello"));
    assert!(!is_palindrome("world"));
}

#[test]
fn test_count_char_integration() {
    // Basic counting
    assert_eq!(count_char("hello world", 'l'), 3);
    assert_eq!(count_char("Mississippi", 's'), 4);
    assert_eq!(count_char("Mississippi", 'i'), 4);
    assert_eq!(count_char("Mississippi", 'p'), 2);

    // Case sensitivity
    assert_eq!(count_char("Hello World", 'H'), 1);
    assert_eq!(count_char("Hello World", 'h'), 0);

    // Character not found
    assert_eq!(count_char("hello", 'x'), 0);
}

#[test]
fn test_reverse_string_integration() {
    // Basic reversal
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("Rust"), "tsuR");

    // Reversal with spaces
    assert_eq!(reverse_string("hello world"), "dlrow olleh");

    // Unicode support
    assert_eq!(reverse_string("hello 👋"), "👋 olleh");
    assert_eq!(reverse_string("café"), "éfac");

    // Empty and single char
    assert_eq!(reverse_string(""), "");
    assert_eq!(reverse_string("a"), "a");
}

#[test]
fn test_combined_string_operations() {
    // Reverse a palindrome should give the same string
    let palindrome = "racecar";
    let reversed = reverse_string(palindrome);
    assert_eq!(palindrome, reversed);
    assert!(is_palindrome(&reversed));

    // Count characters in a reversed string
    let original = "hello";
    let reversed = reverse_string(original);
    assert_eq!(count_char(&original, 'l'), count_char(&reversed, 'l'));
    assert_eq!(count_char(&original, 'h'), count_char(&reversed, 'h'));
}

#[test]
fn test_palindrome_after_reversal() {
    // Non-palindrome becomes palindrome when concatenated with its reverse
    let text = "abc";
    let reversed = reverse_string(text);
    let combined = format!("{}{}", text, reversed);
    assert!(is_palindrome(&combined));
}

#[test]
fn test_count_in_long_strings() {
    // Generate a long string
    let long_string = "a".repeat(1000);
    assert_eq!(count_char(&long_string, 'a'), 1000);
    assert_eq!(count_char(&long_string, 'b'), 0);

    // Mixed characters
    let mixed = "ab".repeat(500);
    assert_eq!(count_char(&mixed, 'a'), 500);
    assert_eq!(count_char(&mixed, 'b'), 500);
}

#[test]
fn test_reverse_preserves_length() {
    let strings = vec!["hello", "world", "Rust programming", ""];
    for s in strings {
        assert_eq!(s.len(), reverse_string(s).len());
    }
}

#[test]
fn test_double_reverse_is_identity() {
    let strings = vec!["hello", "world", "Rust", "12345", "café 👋"];
    for s in strings {
        let reversed_twice = reverse_string(&reverse_string(s));
        assert_eq!(s, reversed_twice);
    }
}
