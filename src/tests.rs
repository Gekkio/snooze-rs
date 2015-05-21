use time::Duration;

use super::Snooze;

#[cfg(test)]
fn average_error(count: i32, duration: Duration) -> Duration {
    let mut total = Duration::zero();
    let mut snooze = Snooze::new(duration).unwrap();

    for _ in 0..count {
        total = total + (Duration::span(|| {
            snooze.wait().unwrap();
        }) - duration);
    }
    total / count
}

#[cfg(test)]
fn test_average_error(duration: Duration) {
    let mut error =
        average_error(16, duration);
    if error < Duration::zero() {
        error = -error;
    }
    // Expect < 1 ms average absolute error.
    // No idea if this is reasonable
    let max_error = Duration::milliseconds(1);
    assert!(error < max_error, "error({}) < max_error({})", error, max_error);
}
#[test]
fn test_average_error_100us() {
    test_average_error(Duration::microseconds(100));
}
#[test]
fn test_average_error_1ms() {
    test_average_error(Duration::milliseconds(1));
}
#[test]
fn test_average_error_10ms() {
    test_average_error(Duration::milliseconds(10));
}
#[test]
fn test_average_error_100ms() {
    test_average_error(Duration::milliseconds(100));
}
