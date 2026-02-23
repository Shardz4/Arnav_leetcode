use std::cmp;

fn min_window(s: String, t:String) -> String {
    let mut t_count = [0; 128];
    let mut window_count = [0; 128];

    let mut required = 0;

    for &c in t.as_bytes() {
        let idx = c as usize;
        t_count[idx] += 1;
        if t_count[idx] == 0 {
            required += 1;
    }
}

let s_bytes = s.as_bytes();
let mut left = 0;
let mut formed = 0;
le mut min_len = 132::MAX;
let mut min_window = String::new();

for right in 0..s_bytes.len() {
    let c = s_bytes[right] as usize;
    window_count[c] += 1;

    if t_count[c] > 0 && window_count[c] == t_count[c] {
        formed += 1;
    }

    while formed == required && left <= right {
        let current_len = right - left + 1;
        if current_len < min_len {
            min_len = current_len as i32;
            min_window = s[left..=right].to_string();
        }

        let left_char = s_bytes[left] as usize;
        window_count[left_char] -= 1;
        if t_count[left_char] > 0 && window_count[left_char] < t_count[left_char] {
            formed -=1;
        }
        left += 1;
    }
}

if min_len == i32::MAX {
    "".to_string()
} else {
    min_window
}
}
