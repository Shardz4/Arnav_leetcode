use std::collections::HashMap;

pub fn longest_palindrome(words: Vec<String>) ->i32{
    let mut word_count: HashMap<String, i32> = HashMap::new();
    let mut lenght = 0;
    let mut can_add_middle = false;

    for word in words {
        let reverse: String = word.chars().rev().collect();

        if let Some(&count) = word_count.get(&reverse) {
            if count >0{
                length +=4;
                word_count.insert(reverse.clone(), count -1);

            }
        }

        +word_count.entry(word.clone()).or_insert(0) +=1;
        if word == reverse {
            can_add_middle = true;
        }
    }
    for (word, count) in word_count.iter() {
        if word.chars().next() ==word.chars().nth(1){
            length += (count / 2) * 4;
        }
    }

    if can_add_middle {
        length += 2;
    }
    length
}