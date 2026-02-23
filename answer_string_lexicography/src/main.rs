use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        let n = word.len();
        let m = num_friends as usize;
        // Edge cases
        if n == 0 || n < m || num_friends <= 0 {
            return String::new();
        }
        if m == 1 {
            return word;
        }

        let chars: Vec<char> = word.chars().collect();
        // Find the lexicographically largest character
        let max_char = *chars.iter().max().unwrap_or(&'\0');

        // HashMap to store substrings starting with max_char, mapping start position to substring
        let mut candidates: HashMap<usize, String> = HashMap::new();

        // Step 1: Find all positions of max_char and collect substrings
        for pos in 0..n {
            if chars[pos] == max_char {
                // Compute the maximum allowed length for the substring
                let max_len = (n - pos).min(n - m + 1);
                if max_len == 0 {
                    continue;
                }
                // Take the substring from pos to pos + max_len
                let substr: String = chars[pos..pos + max_len].iter().collect();
                candidates.insert(pos, substr);
            }
        }

        // Step 2: Find the lexicographically largest substring among candidates
        let mut max_substring = String::new();
        for (_pos, substr) in candidates.iter() {
            if substr > &max_substring {
                max_substring = substr.clone();
            }
        }

        max_substring
    }
}

fn main() {
    let tests = vec![
        ("dbca", 2, "dbc"),
        ("adbcde", 2, "e"),
        ("adeceb", 2, "eceb"),
        ("banana", 3, "nana"),
        ("aabb", 2, "bb"),
        ("dbca", 3, "db"),
    ];

    for (word, k, expected) in tests {
        let result = Solution::answer_string(word.to_string(), k);
        println!("word: {}, k: {}, result: {}, expected: {}", word, k, result, expected);
        assert_eq!(result, expected, "Failed for word: {}, k: {}", word, k);
    }
}