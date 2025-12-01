use chrono::NaiveDate;
use rust_utils_lib::{date_difference, format_date, parse_date, validate_date_format};

#[test]
fn test_date_difference_integration() {
    let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let date2 = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

    let diff = date_difference(&date1, &date2);
    assert_eq!(diff.days, 365);
    assert!(diff.weeks >= 52);
    assert_eq!(diff.years, 1);
}

#[test]
fn test_date_difference_past_to_future() {
    let past = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    let future = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

    let diff = date_difference(&past, &future);
    assert!(diff.days > 1400); // Approximately 4 years
    assert_eq!(diff.years, 4);
}

#[test]
fn test_date_difference_future_to_past() {
    let future = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let past = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();

    let diff = date_difference(&future, &past);
    assert!(diff.days < -1400); // Negative difference
    assert!(diff.years < 0);
}

#[test]
fn test_validate_date_format_integration() {
    // Valid dates in different formats
    assert!(validate_date_format("25/12/2024", "DD/MM/YYYY"));
    assert!(validate_date_format("2024-12-25", "YYYY-MM-DD"));
    assert!(validate_date_format("12/25/2024", "MM/DD/YYYY"));

    // Invalid dates
    assert!(!validate_date_format("32/01/2024", "DD/MM/YYYY")); // Invalid day
    assert!(!validate_date_format("2024-13-01", "YYYY-MM-DD")); // Invalid month
    assert!(!validate_date_format("invalid", "YYYY-MM-DD"));
}

#[test]
fn test_format_date_integration() {
    let date = NaiveDate::from_ymd_opt(2024, 7, 4).unwrap();

    // Test all supported formats
    assert_eq!(
        format_date(&date, "DD/MM/YYYY"),
        Some("04/07/2024".to_string())
    );
    assert_eq!(
        format_date(&date, "YYYY-MM-DD"),
        Some("2024-07-04".to_string())
    );
    assert_eq!(
        format_date(&date, "MM/DD/YYYY"),
        Some("07/04/2024".to_string())
    );
    assert_eq!(
        format_date(&date, "Month DD, YYYY"),
        Some("July 04, 2024".to_string())
    );

    // Invalid format
    assert_eq!(format_date(&date, "INVALID"), None);
}

#[test]
fn test_parse_date_integration() {
    // Parse different formats
    let date1 = parse_date("2024-12-25").unwrap();
    let date2 = parse_date("25/12/2024").unwrap();
    let date3 = parse_date("12/25/2024").unwrap();

    // All should represent Christmas 2024
    let expected = NaiveDate::from_ymd_opt(2024, 12, 25).unwrap();
    assert_eq!(date1, expected);
    assert_eq!(date2, expected);
    assert_eq!(date3, expected);
}

#[test]
fn test_combined_parse_and_format() {
    // Parse a date and format it in different ways
    let date = parse_date("2024-06-15").unwrap();

    assert_eq!(
        format_date(&date, "DD/MM/YYYY"),
        Some("15/06/2024".to_string())
    );
    assert_eq!(
        format_date(&date, "MM/DD/YYYY"),
        Some("06/15/2024".to_string())
    );
    assert_eq!(
        format_date(&date, "Month DD, YYYY"),
        Some("June 15, 2024".to_string())
    );
}

#[test]
fn test_validate_and_parse_workflow() {
    let date_str = "2024-12-25";
    let format = "YYYY-MM-DD";

    // First validate the format
    assert!(validate_date_format(date_str, format));

    // Then parse it
    let parsed = parse_date(date_str).unwrap();
    assert_eq!(parsed, NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());

    // Then format it differently
    let formatted = format_date(&parsed, "DD/MM/YYYY").unwrap();
    assert_eq!(formatted, "25/12/2024");
}

#[test]
fn test_date_difference_with_formatting() {
    let date1 = parse_date("2024-01-01").unwrap();
    let date2 = parse_date("2024-12-31").unwrap();

    let diff = date_difference(&date1, &date2);

    // Format both dates
    let formatted1 = format_date(&date1, "Month DD, YYYY").unwrap();
    let formatted2 = format_date(&date2, "Month DD, YYYY").unwrap();

    assert_eq!(formatted1, "January 01, 2024");
    assert_eq!(formatted2, "December 31, 2024");
    assert_eq!(diff.days, 365);
}

#[test]
fn test_leap_year_handling() {
    // 2024 is a leap year, so February has 29 days
    let feb28 = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
    let feb29 = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    let mar1 = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();

    assert!(validate_date_format("2024-02-29", "YYYY-MM-DD"));

    let diff1 = date_difference(&feb28, &feb29);
    let diff2 = date_difference(&feb29, &mar1);

    assert_eq!(diff1.days, 1);
    assert_eq!(diff2.days, 1);
}

#[test]
fn test_round_trip_formatting() {
    let original = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

    // Format in different styles
    let formatted1 = format_date(&original, "YYYY-MM-DD").unwrap();
    let formatted2 = format_date(&original, "DD/MM/YYYY").unwrap();

    // Parse them back
    let parsed1 = parse_date(&formatted1).unwrap();
    let parsed2 = parse_date(&formatted2).unwrap();

    // Should get the same date
    assert_eq!(parsed1, original);
    assert_eq!(parsed2, original);
}

#[test]
fn test_edge_case_dates() {
    // First day of year
    let jan1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    assert_eq!(
        format_date(&jan1, "YYYY-MM-DD"),
        Some("2024-01-01".to_string())
    );

    // Last day of year
    let dec31 = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    assert_eq!(
        format_date(&dec31, "YYYY-MM-DD"),
        Some("2024-12-31".to_string())
    );

    // Difference should be 365 days (leap year)
    let diff = date_difference(&jan1, &dec31);
    assert_eq!(diff.days, 365);
}
