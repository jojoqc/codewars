/*
When provided with a number between 0-9, return it in words.
Input :: 1
Output :: "One".
If your language supports it, try using a switch statement.
*/

fn switch_it_up(n: usize) -> &'static str {
    match n {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => panic!("Numbers from 0 to 9 only!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(switch_it_up(1), "One");
        assert_eq!(switch_it_up(2), "Two");
        assert_eq!(switch_it_up(3), "Three");
    }
}
