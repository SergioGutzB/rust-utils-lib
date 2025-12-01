mod date_utils;
mod file_io_utils;
mod math_utils;
mod string_utils;

// Re-export all public functions from math_utils
pub use math_utils::{factorial, gcd, is_prime};

// Re-export all public functions from string_utils
pub use string_utils::{count_char, is_palindrome, reverse_string};

// Re-export all public functions and types from date_utils
pub use date_utils::{
    DateDifference, date_difference, format_date, parse_date, validate_date_format,
};

// Re-export all public functions from file_io_utils
pub use file_io_utils::{append_to_file, read_file, write_file};
