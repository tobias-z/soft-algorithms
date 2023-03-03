pub fn arr_includes(arr: Vec<String>, test: String) {
    for ele in arr {
        if ele == test {
            true
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_incorrect_will_fail() {
        let actual = arr_includes(vec!["hello".to_string(), "world".to_string()], "wor".to_string());
        assert_eq!(false, actual);
    }

    #[test]
    fn given_correct_will_succeed() {
        let actual = arr_includes(vec!["hello".to_string(), "world".to_string()], "world".to_string());
        assert_eq!(true, actual);
    }
}
