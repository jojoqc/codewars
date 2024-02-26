fn opposite(number: i32) -> i32 {
    number*-1 as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(opposite(1), -1);
        assert_eq!(opposite(-1), 1);
    }
}