/*
Your task is to make a function that can take any non-negative
integer as an argument and return it with its digits in descending order.
Essentially, rearrange the digits to create the highest possible number.
Examples:
Input: 42145 Output: 54421
Input: 145263 Output: 654321
Input: 123456789 Output: 987654321
*/

fn descending_order(x: u64) -> u64 {
    let a: Vec<_> = x
        .to_string()
        .chars()
        .rev()
        .filter_map(|b| b.to_digit(10))
        .collect();
    let mut c: Vec<u32> = a.clone();
    c.sort_by(|d, e| e.cmp(d));
    c.iter().fold(0, |acc, &digit| acc * 10 + digit as u64)
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
