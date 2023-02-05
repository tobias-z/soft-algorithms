pub fn prime_factorial(n: usize) -> Vec<usize> {
    let mut result = vec![];
    if is_prime(n) {
        result.push(n);
        return result;
    }

    let mut i = 2;
    loop {
        if is_prime(i) {
            let divided_n = n / i;
            if has_decimals(divided_n) {
                i += 1;
                continue;
            }
            result.push(i); // i was a prime number used to divide
            if is_prime(divided_n) {
                result.push(divided_n);
                break;
            }
            i = 2;
        }
    }

    result
}

fn has_decimals(n: usize) -> bool {
    n % 1 != 0
}

// a prime number has only two factors. The number itself and 1.
fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_prime_factorial() {
        assert_eq!(prime_factorial(12), vec![2, 2, 3]);
        assert_eq!(prime_factorial(147), vec![3, 7, 7]);
    }

    #[test]
    fn can_find_prime_number() {
        assert!(is_prime(2));
    }
}
