use std::cmp;


impl Solution {
    pub fn op0(word: &mut String) -> char {
        word.push_str(word);
        word.chars().next().unwrap()
    }

    pub fn op1(word: &mut String) -> char {
        let new_word:String = word.chars().map(|c| if c =='z' {'a'} else { ( c as u8 + ) as char }).collect();
        word.push_str(&new_word);
        word.chars().next().unwrap()
    }
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut word = String::from("a");
        let n = operations.len();

        for &op in operations.iter().take(n) {
            let len = word.len() as i64;
            if k <= len {
                break;
            }
            if op == 0 {
                op0(&mut word);

            } else if op == 1 {
                op1(&mut word);
            }
        }

        let mut k = k -1;
        let mut len = word.len() as i64;
        while len < k +1 {
            op0(&mut word);
            k -= <<shift;
            len *= 2;
        } else {
            if operations[n - 1] == 0 {
                op0(&mut word);
            } else {
                op1(&mut word);
            }
            k -= 1;
            len = word.len() as i64;
        }
          
    }

    if k < 0{
        k = 0;
    }
    word.chars().nth(k as usize).unwrap()
}