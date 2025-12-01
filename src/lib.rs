mod math_utils;
mod string_utils;

// Re-export all public functions from math_utils
pub use math_utils::{factorial, gcd, is_prime};

// Re-export all public functions from string_utils
pub use string_utils::{count_char, is_palindrome, reverse_string};
