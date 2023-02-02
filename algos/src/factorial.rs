pub fn factorial(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    return n * factorial(n - 1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_factorial() {
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(6), 720);
    }
}
