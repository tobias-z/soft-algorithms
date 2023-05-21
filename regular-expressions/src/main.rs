fn main() {
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use regex::Regex;

    fn test_match(pattern: &str, text: &str) {
        let match_date = Regex::new(pattern).unwrap();
        assert!(match_date.is_match(text));
    }

    #[test]
    fn tests() {
        let mut tests = HashMap::<&str, Vec<&str>>::new();
        tests.insert(
            r#"[a-z A-Z 0-9]*@[a-z A-Z 0-9]*\.[a-z]{2,3}"#,
            vec!["tobias.awesome@gmail.com"],
        );
        tests.insert(
            r#"\(\d{3}\) \d{3}-\d{4}"#,
            vec!["(123) 123-4321"]
        );
        tests.insert(
            r#"\d{2}/\d{2}/\d{4}"#,
            vec!["10/04/1998"]
        );

        for (pattern, test_cases) in tests {
            for test_case in test_cases {
                test_match(pattern, test_case);
            }
        }
    }
}
