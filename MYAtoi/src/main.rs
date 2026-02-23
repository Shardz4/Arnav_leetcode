pub fn my_atoi(s: String) -> i32 {
    const MAX: i64 = i32::MAX as i64;
    const MIN: i64 = i32::MIN as i64;

    let s = s.trim_start();
    if s.is_empty() {
        return 0;
    }

    let mut chars = s.chars();
    let mut sign = 1;

    let first = chars.next();
    if first == Some('-') {
        sign = -1;
    } else if first != Some('+') {
        chars = s.chars();
    }

    let mut result: i64 = 0;
    for c in chars {
        if !c.is_digit(10) {
            break;
        }
        let digit = (c as u8 - b'0') as i64;
        result = result * 10 + digit;

        if result * sign > MAX {
            return i32::MAX;
        }
        if result * sign < MIN {
            return i32::MIN;
        }
    }

    (result * sign) as i32
}

fn main() {
    let test_cases = vec![
        "42",
        "   -42",
        "4193 with words",
        "words and 987",
        "-91283472332",
        "",
        "+1",
        "  00042",
        "2147483648",
        "-2147483649",
        "+-12",
        "   ",
    ];

    for s in test_cases {
        let result = my_atoi(s.to_string());
        println!("Input: {}, Output: {}", s, result);
    }
}