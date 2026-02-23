use std :: collections::HashMap;
use std :: cmp::max;

pub fn lenght_of_longest_substring(s: String) -> i32 {
    let mut char_index: HashMap<char, usize> = HashMap::new();
    let mut max_lenght = 0;
    let mut left = 0;

    for (right, char) in s.chars().enumerate() {
        if let Some (&prev_index)= char_index.get(&char){
            if prev_index >= left {
                left = prev_index + 1;
            }
        }
        max_lenght = max(max_lenght, right - left + 1);
        char_index.insert(char, right);
    }
    max_lenght as i32
}

fn main() {
    let s = String::from("abcabcbb");
    let a = String::from("bbbbb");
    let b = String::from("pwwkew");
    let c = String::from(" ");
    let mut result = lenght_of_longest_substring(s);
    println!("The length of the longest substring without repeating characters is: {}", result);
    result = lenght_of_longest_substring(a);
    println!("The length of the longest substring without repeating characters is: {}", result);
    result = lenght_of_longest_substring(b);
    println!("The length of the longest substring without repeating characters is: {}", result);
    result = lenght_of_longest_substring(c);
    println!("The length of the longest substring without repeating characters is: {}", result);
}