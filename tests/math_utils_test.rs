use rust_utils_lib::{factorial, gcd, is_prime};

#[test]
fn test_factorial_integration() {
    // Test basic factorial calculations
    assert_eq!(factorial(0), Some(1));
    assert_eq!(factorial(5), Some(120));
    assert_eq!(factorial(10), Some(3_628_800));

    // Test overflow boundary
    assert_eq!(factorial(20), Some(2_432_902_008_176_640_000));
    assert_eq!(factorial(21), None);
}

#[test]
fn test_gcd_integration() {
    // Test basic GCD calculations
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(100, 50), 50);

    // Test commutativity
    assert_eq!(gcd(54, 24), gcd(24, 54));

    // Test with coprime numbers
    assert_eq!(gcd(17, 19), 1);

    // Test with zero
    assert_eq!(gcd(0, 42), 42);
    assert_eq!(gcd(42, 0), 42);
}

#[test]
fn test_is_prime_integration() {
    // Test small primes
    let small_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for &prime in &small_primes {
        assert!(is_prime(prime), "{} should be prime", prime);
    }

    // Test composites
    let composites = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18];
    for &composite in &composites {
        assert!(!is_prime(composite), "{} should not be prime", composite);
    }

    // Test edge cases
    assert!(!is_prime(0));
    assert!(!is_prime(1));

    // Test larger prime
    assert!(is_prime(97));
}

#[test]
fn test_combined_math_operations() {
    // Find factorial and check if result is a specific value
    let fact_5 = factorial(5).unwrap();
    assert_eq!(fact_5, 120);

    // Find GCD of two factorials
    let fact_4 = factorial(4).unwrap();
    let fact_6 = factorial(6).unwrap();
    assert_eq!(gcd(fact_4, fact_6), fact_4); // 4! divides 6!

    // Check primality of small numbers
    for i in 2..20 {
        if is_prime(i) {
            // Prime numbers should have GCD of 1 with any non-multiple
            for j in 2..i {
                if i % j != 0 {
                    assert_eq!(gcd(i, j), 1, "{} and {} should be coprime", i, j);
                }
            }
        }
    }
}

#[test]
fn test_factorial_sequence() {
    // Verify factorial sequence property: n! = n * (n-1)!
    for n in 2..=10 {
        let fact_n = factorial(n).unwrap();
        let fact_n_minus_1 = factorial(n - 1).unwrap();
        assert_eq!(fact_n, n * fact_n_minus_1);
    }
}

#[test]
fn test_gcd_properties() {
    // GCD(a, b) should divide both a and b
    let a = 48u64;
    let b = 18u64;
    let result = gcd(a, b);

    assert_eq!(a % result, 0, "GCD should divide a");
    assert_eq!(b % result, 0, "GCD should divide b");

    // GCD should be commutative
    assert_eq!(gcd(a, b), gcd(b, a));

    // GCD(a, a) should equal a
    assert_eq!(gcd(a, a), a);
}

#[test]
fn test_prime_density() {
    // Count primes in first 100 numbers (should be 25)
    let prime_count = (2..=100).filter(|&n| is_prime(n)).count();
    assert_eq!(prime_count, 25);
}
