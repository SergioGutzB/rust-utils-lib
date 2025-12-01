use chrono::{NaiveDate, ParseError};

/// Represents the difference between two dates.
#[derive(Debug, PartialEq, Eq)]
pub struct DateDifference {
    pub days: i64,
    pub weeks: i64,
    pub years: i64,
}

/// Calculate the difference between two dates.
///
/// Returns a `DateDifference` struct containing the difference in days, weeks, and years.
/// The difference is calculated as `date2 - date1`, so a positive result means date2 is later.
///
/// # Examples
///
/// ```
/// use rust_utils_lib::date_difference;
/// use chrono::NaiveDate;
///
/// let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// let date2 = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
/// let diff = date_difference(&date1, &date2);
///
/// assert_eq!(diff.days, 365);
/// assert!(diff.weeks > 50);
/// ```
pub fn date_difference(date1: &NaiveDate, date2: &NaiveDate) -> DateDifference {
    let days = (*date2 - *date1).num_days();
    let weeks = days / 7;
    let years = days / 365; // Approximate, not accounting for leap years

    DateDifference { days, weeks, years }
}

/// Validate if a string matches a specific date format.
///
/// Supported formats:
/// - "DD/MM/YYYY"
/// - "YYYY-MM-DD"
/// - "MM/DD/YYYY"
///
/// # Examples
///
/// ```
/// use rust_utils_lib::validate_date_format;
///
/// assert!(validate_date_format("25/12/2024", "DD/MM/YYYY"));
/// assert!(validate_date_format("2024-12-25", "YYYY-MM-DD"));
/// assert!(!validate_date_format("2024/12/25", "DD/MM/YYYY"));
/// assert!(!validate_date_format("invalid", "YYYY-MM-DD"));
/// ```
pub fn validate_date_format(date_str: &str, format: &str) -> bool {
    let chrono_format = match format {
        "DD/MM/YYYY" => "%d/%m/%Y",
        "YYYY-MM-DD" => "%Y-%m-%d",
        "MM/DD/YYYY" => "%m/%d/%Y",
        _ => return false,
    };

    NaiveDate::parse_from_str(date_str, chrono_format).is_ok()
}

/// Format a date in different styles.
///
/// Supported output formats:
/// - "DD/MM/YYYY" (e.g., "25/12/2024")
/// - "YYYY-MM-DD" (e.g., "2024-12-25")
/// - "MM/DD/YYYY" (e.g., "12/25/2024")
/// - "Month DD, YYYY" (e.g., "December 25, 2024")
///
/// # Examples
///
/// ```
/// use rust_utils_lib::format_date;
/// use chrono::NaiveDate;
///
/// let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
///
/// assert_eq!(format_date(&date, "DD/MM/YYYY"), Some("25/12/2024".to_string()));
/// assert_eq!(format_date(&date, "YYYY-MM-DD"), Some("2024-12-25".to_string()));
/// assert_eq!(format_date(&date, "MM/DD/YYYY"), Some("12/25/2024".to_string()));
/// assert_eq!(format_date(&date, "Month DD, YYYY"), Some("December 25, 2024".to_string()));
/// assert_eq!(format_date(&date, "INVALID"), None);
/// ```
pub fn format_date(date: &NaiveDate, format: &str) -> Option<String> {
    let chrono_format = match format {
        "DD/MM/YYYY" => "%d/%m/%Y",
        "YYYY-MM-DD" => "%Y-%m-%d",
        "MM/DD/YYYY" => "%m/%d/%Y",
        "Month DD, YYYY" => "%B %d, %Y",
        _ => return None,
    };

    Some(date.format(chrono_format).to_string())
}

/// Parse a date string in various common formats.
///
/// Attempts to parse the date using multiple common formats.
///
/// # Examples
///
/// ```
/// use rust_utils_lib::parse_date;
/// use chrono::NaiveDate;
///
/// let date = parse_date("2024-12-25").unwrap();
/// assert_eq!(date, NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
///
/// let date = parse_date("25/12/2024").unwrap();
/// assert_eq!(date, NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
/// ```
pub fn parse_date(date_str: &str) -> Result<NaiveDate, ParseError> {
    // Try different formats
    let formats = vec!["%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y"];

    for format in formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(date);
        }
    }

    // If none worked, try the first format again to get the error
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for date_difference
    #[test]
    fn test_date_difference_basic() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();

        let diff = date_difference(&date1, &date2);
        assert_eq!(diff.days, 7);
        assert_eq!(diff.weeks, 1);
        assert_eq!(diff.years, 0);
    }

    #[test]
    fn test_date_difference_negative() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let diff = date_difference(&date1, &date2);
        assert_eq!(diff.days, -7);
        assert_eq!(diff.weeks, -1);
    }

    #[test]
    fn test_date_difference_year() {
        let date1 = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let diff = date_difference(&date1, &date2);
        assert_eq!(diff.days, 365); // From 2023-01-01 to 2024-01-01 is 365 days
        assert_eq!(diff.years, 1);
    }

    #[test]
    fn test_date_difference_same_date() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let diff = date_difference(&date, &date);
        assert_eq!(diff.days, 0);
        assert_eq!(diff.weeks, 0);
        assert_eq!(diff.years, 0);
    }

    // Tests for validate_date_format
    #[test]
    fn test_validate_date_format_dd_mm_yyyy() {
        assert!(validate_date_format("25/12/2024", "DD/MM/YYYY"));
        assert!(validate_date_format("01/01/2024", "DD/MM/YYYY"));
        assert!(!validate_date_format("2024-12-25", "DD/MM/YYYY"));
        assert!(!validate_date_format("32/13/2024", "DD/MM/YYYY")); // Invalid date
    }

    #[test]
    fn test_validate_date_format_yyyy_mm_dd() {
        assert!(validate_date_format("2024-12-25", "YYYY-MM-DD"));
        assert!(validate_date_format("2024-01-01", "YYYY-MM-DD"));
        assert!(!validate_date_format("25/12/2024", "YYYY-MM-DD"));
        assert!(!validate_date_format("2024-13-32", "YYYY-MM-DD")); // Invalid date
    }

    #[test]
    fn test_validate_date_format_mm_dd_yyyy() {
        assert!(validate_date_format("12/25/2024", "MM/DD/YYYY"));
        assert!(validate_date_format("01/01/2024", "MM/DD/YYYY"));
        assert!(!validate_date_format("2024-12-25", "MM/DD/YYYY"));
    }

    #[test]
    fn test_validate_date_format_invalid_format() {
        assert!(!validate_date_format("25/12/2024", "INVALID"));
        assert!(!validate_date_format("invalid", "DD/MM/YYYY"));
    }

    // Tests for format_date
    #[test]
    fn test_format_date_dd_mm_yyyy() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert_eq!(
            format_date(&date, "DD/MM/YYYY"),
            Some("25/12/2024".to_string())
        );
    }

    #[test]
    fn test_format_date_yyyy_mm_dd() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert_eq!(
            format_date(&date, "YYYY-MM-DD"),
            Some("2024-12-25".to_string())
        );
    }

    #[test]
    fn test_format_date_mm_dd_yyyy() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert_eq!(
            format_date(&date, "MM/DD/YYYY"),
            Some("12/25/2024".to_string())
        );
    }

    #[test]
    fn test_format_date_month_dd_yyyy() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert_eq!(
            format_date(&date, "Month DD, YYYY"),
            Some("December 25, 2024".to_string())
        );
    }

    #[test]
    fn test_format_date_invalid_format() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
        assert_eq!(format_date(&date, "INVALID"), None);
    }

    // Tests for parse_date
    #[test]
    fn test_parse_date_yyyy_mm_dd() {
        let result = parse_date("2024-12-25").unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
    }

    #[test]
    fn test_parse_date_dd_mm_yyyy() {
        let result = parse_date("25/12/2024").unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
    }

    #[test]
    fn test_parse_date_mm_dd_yyyy() {
        let result = parse_date("12/25/2024").unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!(parse_date("invalid").is_err());
        assert!(parse_date("32/13/2024").is_err());
    }
}
