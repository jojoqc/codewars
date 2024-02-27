/*
The cockroach is one of the fastest insects. Write a function which takes its speed in km per hour and returns it in cm per second, rounded down to the integer (= floored).
For example:
1.08 --> 30
Note! The input is a Real number (actual type is language dependent) and is >= 0. The result should be an Integer.
*/

fn cockroach_speed(s: f64) -> i64 {
    ((s * 1000.0) / 36.0).floor() as i64
}

#[test]
fn basic_tests() {
    assert_eq!(cockroach_speed(1.08), 30);
    assert_eq!(cockroach_speed(1.09), 30);
    assert_eq!(cockroach_speed(0.0), 0);
}
