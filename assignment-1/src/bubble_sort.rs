pub fn bubble_sort(arr: &mut Vec<usize>) {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_sort() {
        let mut arr = vec![100, 20, 100, 200, 1, 0, 1, 2, 1, 10, 40, 100, 0];
        bubble_sort(&mut arr);
        println!("{:?}", arr);
    }
}
