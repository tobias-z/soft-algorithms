// 1. O(n)
// 2. O(n)
// 3. O(n log n)
// 4. O(1)
// 5. O(n)
// 6. O(n)
// 7. O(n)
// 8. O(n)
// 9. O(n)
// 10. O(n)
// 11. O(n!)

// sort array where smallest is in front
fn sort(mut arr: Vec<usize>) -> Vec<usize> {
    for i in 0..arr.len() {
        if i == 0 {
            continue;
        }
        let prev = arr[i - 1];
        if arr[i] < prev {
            arr[i - 1] = arr[i];
            arr[i] = prev
        }
    }
    arr
}

fn bubble_sort(mut arr: Vec<usize>) -> Vec<usize> {
    for _i in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                let curr = arr[j];
                let prev = arr[j + 1];
                arr[j] = prev;
                arr[j + 1] = curr;
            }
        }
    }
    arr
}

fn string_number_bubble_sort(mut arr: Vec<String>) -> Vec<String> {
    for _i in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            let curr = arr.get(j).unwrap().to_owned();
            let next = arr.get(j + 1).unwrap().to_owned();
            let curr_is_n = is_number(&curr);
            let next_is_n = is_number(&next);
            if curr_is_n && next_is_n && arr[j] > arr[j + 1] {
                arr[j] = next.clone();
                arr[j + 1] = curr.clone();
            }
            if !curr_is_n && next_is_n {
                arr[j] = next.clone();
                arr[j + 1] = curr.clone();
            }
            if curr_is_n && !next_is_n {
                continue;
            }
            if !curr_is_n
                && !next_is_n
                && curr.chars().next().unwrap() > next.chars().next().unwrap()
            {
                arr[j] = next.clone();
                arr[j + 1] = curr.clone();
            }
        }
    }
    arr
}

fn is_number(curr: &str) -> bool {
    for c in curr.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_sort() {
        let list = bubble_sort(vec![1, 3, 5, 2, 1, 4]);
        println!("{:?}", list);
    }

    #[test]
    fn can_string_number_sort_first() {
        let sorted = string_number_bubble_sort(vec![
            "ABC".to_string(),
            "FCA".to_string(),
            "A".to_string(),
            "S".to_string(),
            "0".to_string(),
            "12".to_string(),
            "25".to_string(),
            "1".to_string(),
            "hej med dig".to_string(),
            "Bob".to_string(),
        ]);

        println!("{:?}", sorted);
    }
}
