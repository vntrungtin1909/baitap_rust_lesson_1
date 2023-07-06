// Exercise 1
#[allow(dead_code)]
fn exercise1(color: &str) -> String {
    match color {
        "white" => "white".to_string(),
        "green" => "mau xanh".to_string(),
        "blue" => "xanh diff".to_string(),
        "cyan" => "xanh diff diff".to_string(),
        _=> "khong phai mau xanh".to_string(),
    }
}

// Exercise 2
// Fix all errors without adding newline
fn exercise2() -> String {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s.push('!');
    s
}
// Exercise 3
// Fix errors without removing any line
fn exercise3() -> String {
    let s1 = String::from("hello,");
    let s2 = String::from(" world!");
    let s3 = s1 + &s2;
    s3
}

// Exercise 4
// Reverse a string

fn reverse_string(input: &str) -> String {
    let reversed_chars: Vec<char> = input.chars().rev().collect();  
    let reversed_string: String = reversed_chars.into_iter().collect();  
    reversed_string
}


// Exercise 5
// Check if a string is a palindrome
fn is_palindrome(word: &str) -> bool {
        let word = word.chars().collect::<Vec<char>>();  
        let len = word.len();
        for i in 0..len / 2 {
            if word[i] != word[len - 1 - i] {
                return false;
            }
        }
        true
}

// Exercise 6
// Count the occurrences of a character in a string
fn count_char_occurrences(string: &str, ch: char) -> usize {
    let mut count = 0;
    for c in string.chars() {
        if c == ch {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn exercise1_work() {
        assert_eq!("white".to_string(), exercise1("white"));
    }

    // Test for exercise 2
    #[test]
    fn exercise2_work() {
        assert_eq!("hello, world!".to_string(), exercise2());
    }

    // Test for exercise 3
    #[test]
    fn exercise3_work() {
        assert_eq!("hello, world!".to_string(), exercise3());
    }
    
    // Test for exercise 4
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("world"), "dlrow");
        assert_eq!(reverse_string(""), "");
    }

    // Test for exercise 5
    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("level"), true);
        assert_eq!(is_palindrome("deed"), true);
        assert_eq!(is_palindrome("rotor"), true);
    }
    // Test for exercise 5
    #[test]
    fn test_non_palindrome() {
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("world"), false);
    }

    // Test for exercise 6

    #[test]
    fn test_count_char_occurrences() {
        assert_eq!(count_char_occurrences("Hello", 'l'), 2);
        assert_eq!(count_char_occurrences("Rust is fun", 'u'), 2);
        assert_eq!(count_char_occurrences("Mississippi", 's'), 4);
    }

}