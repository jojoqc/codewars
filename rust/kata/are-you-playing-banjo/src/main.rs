fn are_you_playing_banjo(name: &str) -> String {
    if name.chars().nth(0).unwrap().to_ascii_lowercase() == 'r'{
        return name.to_owned() + " plays banjo";
    }
    name.to_owned() + " does not play banjo"
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
