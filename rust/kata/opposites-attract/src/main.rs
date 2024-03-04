/*
Timmy & Sarah think they are in love, but around where they live,
they will only know once they pick a flower each.
If one of the flowers has an even number of petals and the other
has an odd number of petals it means they are in love.

Write a function that will take the number of petals of each flower and return true
if they are in love and false if they aren't.
*/

fn lovefunc(flower1: u16, flower2: u16) -> bool {
    (flower1 + flower2) % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test(1, 4, true);
        do_test(2, 2, false);
        do_test(0, 1, true);
        do_test(0, 0, false);
    }

    fn do_test(f1: u16, f2: u16, exp: bool) {
        assert_eq!(
            lovefunc(f1, f2),
            exp,
            "\nFailed with flower1 {}, flower2 {}",
            f1,
            f2
        );
    }
}
