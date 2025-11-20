use std::{cmp::min, io};

fn longest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut temp_s = String::with_capacity(2 * s.len() + 1);

    temp_s.push('^');
    temp_s.push('#');
    for c in s.chars() {
        temp_s.push(c);
        temp_s.push('#');
    }
    temp_s.push('$');

    let n = temp_s.len();
    let temp_s_bytes = temp_s.as_bytes();
    let mut radiuses = vec![0; n];
    let mut current_center = 0;
    let mut current_right_border = 0;
    let mut center = 0;
    let mut max_length = 0;

    for i in 1..n - 1 {
        if i >= current_right_border {
            while i + radiuses[i] + 1 < n
                && i > radiuses[i]
                && temp_s_bytes[i - radiuses[i] - 1] == temp_s_bytes[i + radiuses[i] + 1]
            {
                radiuses[i] += 1;
            }

            if i + radiuses[i] > current_right_border {
                current_center = i;
                current_right_border = current_center + radiuses[i];
            }

            if radiuses[i] > max_length {
                max_length = radiuses[i];
                center = i;
            }
        } else {
            let i_mirror = 2 * current_center - i;

            radiuses[i] = min(current_right_border - i, radiuses[i_mirror]);

            while i + radiuses[i] + 1 < n
                && i > radiuses[i]
                && temp_s_bytes[i - radiuses[i] - 1] == temp_s_bytes[i + radiuses[i] + 1]
            {
                radiuses[i] += 1;
            }

            if i + radiuses[i] > current_right_border {
                current_center = i;
                current_right_border = current_center + radiuses[i];
            }

            if radiuses[i] > max_length {
                max_length = radiuses[i];
                center = i;
            }
        }
    }

    let start = (center - max_length) / 2;
    s.chars().skip(start).take(max_length).collect()
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let palindrome = longest_palindrome(input.trim());

    println!("{}", palindrome);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_1() {
        assert!(longest_palindrome("akasrjbbaacaabbkamasd") == "bbaacaabb");
    }

    #[test]
    fn test_reference_2() {
        assert!(
            longest_palindrome(
                "if i work then everything will be brilliant we keep the bar even if the conversation is not about statics"
            ) == "illi"
        );
    }

    #[test]
    fn test_empty_string() {
        assert!(longest_palindrome("") == "");
    }

    #[test]
    fn test_single_char() {
        assert!(longest_palindrome("a") == "a");
    }

    #[test]
    fn test_two_chars() {
        assert!(longest_palindrome("ab") == "a");
    }

    #[test]
    fn test_palindrome_odd() {
        assert!(longest_palindrome("aba") == "aba");
    }

    #[test]
    fn test_palindrome_even() {
        assert!(longest_palindrome("abba") == "abba");
    }

    #[test]
    fn test_longest_palindrome_in_middle() {
        assert!(longest_palindrome("babad") == "bab");
    }

    #[test]
    fn test_longest_palindrome_at_end() {
        assert!(longest_palindrome("cbbd") == "bb");
    }

    #[test]
    fn test_longer_palindrome() {
        assert!(longest_palindrome("racecar") == "racecar");
    }

    #[test]
    fn test_palindrome_with_spaces() {
        assert!(longest_palindrome("a b a") == "a b a");
    }

    #[test]
    fn test_nested_palindromes() {
        assert!(longest_palindrome("abacabadabacaba") == "abacabadabacaba");
    }

    #[test]
    fn test_palindrome_number_string() {
        assert!(longest_palindrome("1234567890987654321") == "1234567890987654321");
    }

    #[test]
    fn test_no_palindrome_longer_than_one() {
        assert!(longest_palindrome("abcdefg") == "a");
    }

    #[test]
    fn test_all_same_chars() {
        assert!(longest_palindrome("aaaa") == "aaaa");
    }

    #[test]
    fn test_palindrome_with_newline() {
        assert!(longest_palindrome("aabaa\n") == "aabaa");
    }

    #[test]
    fn test_palindrome_with_crlf() {
        assert!(longest_palindrome("aabaa\r\n") == "aabaa");
    }

    #[test]
    fn test_complex_palindrome() {
        assert!(longest_palindrome("xababay") == "ababa");
    }

    #[test]
    fn test_palindrome_at_start() {
        assert!(longest_palindrome("abcdeedcbax") == "abcdeedcba");
    }

    #[test]
    fn test_palindrome_at_end() {
        assert!(longest_palindrome("xabcdeedcba") == "abcdeedcba");
    }

    #[test]
    fn test_palindrome_entire_string() {
        assert!(longest_palindrome("abcdeedcba") == "abcdeedcba");
    }
}
