/*
Write a function to convert a name into initials. This kata strictly takes two words with one space in between them.
The output should be two capital letters with a dot separating them.
It should look like this:
Sam Harris => S.H
patrick feeney => P.F
*/
fn abbrev_name(name: &str) -> String {
    // code away
    let x: Vec<_> = name.split_whitespace().map(|c| c.to_string()).collect();
    x[0].chars()
        .nth(0)
        .unwrap_or_default()
        .to_string()
        .to_uppercase()
        + "."
        + &x[1]
            .chars()
            .nth(0)
            .unwrap_or_default()
            .to_string()
            .to_uppercase()
}

// Rust test example:
#[test]
fn sample_tests() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
}
