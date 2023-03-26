use std::collections::{BinaryHeap, HashMap};

const IGNORED_CHARS: [char; 5] = [' ', '.', ',', '!', '\n'];
const LOWERCASE_A: u8 = 97;
const LOWERCASE_Z: u8 = 122;

fn get_encrypted(filename: &str) -> String {
    std::fs::read_to_string(format!("documents/{}", filename)).unwrap()
}

fn get_letter_priority_queue(text: &str) -> BinaryHeap<(usize, char)> {
    text
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut occurrences, c| {
            match occurrences.get_mut(&c) {
                Some(n) => {
                    *n += 1;
                }
                None => {
                    occurrences.insert(c, 1);
                }
            };
            occurrences
        })
        .iter()
        .filter(|(key, _)| !IGNORED_CHARS.contains(key))
        .map(|(key, val)| {
            (*val, *key)
        })
        .collect()
}

fn get_key(encrypted: char, assumption: char) -> i32 {
    encrypted as i32 - assumption as i32
}

fn lowercase_wrapping_sub(val: u8, minus_val: i32) -> u8 {
    let subtracted = val as i32 - minus_val;
    if subtracted >= LOWERCASE_A as i32 && subtracted <= LOWERCASE_Z as i32 {
        subtracted as u8
    } else if subtracted < LOWERCASE_A as i32 {
        let upto = LOWERCASE_A - subtracted as u8 - 1;
        return LOWERCASE_Z - upto;
    } else {
        let downto = subtracted as u8 - LOWERCASE_Z - 1;
        return LOWERCASE_A + downto;
    }
}

fn decrypt_from_key(encrypted: &str, key: i32) -> String {
    encrypted.chars()
        .map(|char| {
            if IGNORED_CHARS.contains(&char) {
                return char;
            }
            lowercase_wrapping_sub(char as u8, key) as char
        })
        .collect()
}

pub fn decrypt(filename: &str) {
    let encrypted = get_encrypted(filename);
    let encrypted = encrypted.trim_end();
    let mut occurrences = get_letter_priority_queue(encrypted);
    let most_common = occurrences.pop().unwrap().1;
    let second = occurrences.pop().unwrap().1;
    let third = occurrences.pop().unwrap().1;
    println!("'e' - most common: {}", decrypt_from_key(encrypted, get_key(most_common, 'e')));
    println!("'e' - second most common: {}", decrypt_from_key(encrypted, get_key(second, 'e')));
    println!("'e' - third most common: {}", decrypt_from_key(encrypted, get_key(third, 'e')));
    println!("'t' - most common: {}", decrypt_from_key(encrypted, get_key(most_common, 't')));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decrypt_1() {
        decrypt("Crypt1.txt");
    }

    #[test]
    fn decrypt_2() {
        decrypt("Crypt2.txt");
    }

    #[test]
    fn decrypt_3() {
        decrypt("Crypt3.txt");
    }
}
