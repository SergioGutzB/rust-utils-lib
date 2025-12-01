/// Check if a string is a palindrome.
///
/// A palindrome is a string that reads the same forward and backward,
/// ignoring case, whitespace, and punctuation.
///
/// # Examples
///
/// ```
/// use rust_utils_lib::is_palindrome;
///
/// let text = "A man a plan a canal Panama";
/// if is_palindrome(text) {
///     println!("'{}' is a palindrome!", text);
/// }
///
/// assert!(is_palindrome("racecar"));
/// assert!(is_palindrome("A man a plan a canal Panama"));
/// assert!(!is_palindrome("hello"));
/// assert!(is_palindrome(""));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    cleaned == cleaned.chars().rev().collect::<String>()
}

/// Count the occurrences of a specific character in a string.
///
/// This function is case-sensitive by default.
///
/// # Examples
///
/// ```
/// use rust_utils_lib::count_char;
///
/// let text = "hello world";
/// let count = count_char(text, 'l');
/// assert_eq!(count, 3);
///
/// assert_eq!(count_char("Mississippi", 's'), 4);
/// assert_eq!(count_char("Rust", 'R'), 1);
/// assert_eq!(count_char("Rust", 'r'), 0); // Case-sensitive
/// ```
pub fn count_char(s: &str, target: char) -> usize {
    s.chars().filter(|&c| c == target).count()
}

/// Reverse a string, preserving UTF-8 character boundaries.
///
/// # Examples
///
/// ```
/// use rust_utils_lib::reverse_string;
///
/// let reversed = reverse_string("hello");
/// assert_eq!(reversed, "olleh");
///
/// assert_eq!(reverse_string("Rust"), "tsuR");
/// assert_eq!(reverse_string(""), "");
/// assert_eq!(reverse_string("a"), "a");
/// // Works with Unicode
/// assert_eq!(reverse_string("hello 👋"), "👋 olleh");
/// ```
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for is_palindrome
    #[test]
    fn test_palindrome_simple() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("noon"));
        assert!(is_palindrome("level"));
    }

    #[test]
    fn test_palindrome_with_spaces() {
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome("race car"));
        assert!(is_palindrome("taco cat"));
    }

    #[test]
    fn test_palindrome_case_insensitive() {
        assert!(is_palindrome("RaceCar"));
        assert!(is_palindrome("Madam"));
        assert!(is_palindrome("Was it a car or a cat I saw"));
    }

    #[test]
    fn test_palindrome_with_punctuation() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome("No 'x' in Nixon"));
    }

    #[test]
    fn test_palindrome_edge_cases() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome(" "));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("world"));
        assert!(!is_palindrome("Rust programming"));
    }

    // Tests for count_char
    #[test]
    fn test_count_char_basic() {
        assert_eq!(count_char("hello", 'l'), 2);
        assert_eq!(count_char("hello world", 'o'), 2);
        assert_eq!(count_char("Mississippi", 's'), 4);
    }

    #[test]
    fn test_count_char_case_sensitive() {
        assert_eq!(count_char("Hello", 'H'), 1);
        assert_eq!(count_char("Hello", 'h'), 0);
        assert_eq!(count_char("Rust", 'R'), 1);
        assert_eq!(count_char("Rust", 'r'), 0);
    }

    #[test]
    fn test_count_char_not_found() {
        assert_eq!(count_char("hello", 'x'), 0);
        assert_eq!(count_char("world", 'a'), 0);
    }

    #[test]
    fn test_count_char_empty_string() {
        assert_eq!(count_char("", 'a'), 0);
    }

    #[test]
    fn test_count_char_all_same() {
        assert_eq!(count_char("aaaa", 'a'), 4);
        assert_eq!(count_char("111", '1'), 3);
    }

    // Tests for reverse_string
    #[test]
    fn test_reverse_string_basic() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("Rust"), "tsuR");
        assert_eq!(reverse_string("12345"), "54321");
    }

    #[test]
    fn test_reverse_string_with_spaces() {
        assert_eq!(reverse_string("hello world"), "dlrow olleh");
        assert_eq!(reverse_string("a b c"), "c b a");
    }

    #[test]
    fn test_reverse_string_edge_cases() {
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
        assert_eq!(reverse_string(" "), " ");
    }

    #[test]
    fn test_reverse_string_palindrome() {
        let palindrome = "racecar";
        assert_eq!(reverse_string(palindrome), palindrome);
    }

    #[test]
    fn test_reverse_string_unicode() {
        assert_eq!(reverse_string("hello 👋"), "👋 olleh");
        assert_eq!(reverse_string("café"), "éfac");
        assert_eq!(reverse_string("日本"), "本日");
    }
}
