/*
Return the number (count) of vowels in the given string.
We will consider a, e, i, o, u as vowels for this Kata (but not y).
The input string will only consist of lower case letters and/or spaces.
*/

fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    for x in string.chars() {
        match x {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels_count += 1,
            _ => (),
        };
    }
    vowels_count
}
#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}
