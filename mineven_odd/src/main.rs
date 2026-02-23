impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        if k <= 0 {
            return 0;
        }

        let k = k as usize;
        let chars: Vec<char> = s.chars().collect();
        let mut best = i32::MIN;

        for i in 0..=chars.len().saturating_sub(k) {
            let mut freq = vec![0; 10];
            for j in i..i+k {
                freq[(chars[j] as u8 - b'0') as usize] += 1;
            }

            let mut diff = i32::MIN;
            for j in i..chars.len() {
                if j >= i+k {
                    freq[(chars[j] as u8 - b'0') as usize] += 1;
                }

                let mut max_odd = i32::MIN;
                let mut min_even = i32::MAX;
                for &f in &freq {
                    if f % 2 == 1 {
                        max_odd = max_odd.max(f);
                    } else if f > 0 {
                        min_even = min_even.min(f);
                    }
                }

                if max_odd != i32::MIN && min_even != i32::MAX {
                    diff = diff.max(max_odd - min_even);
                }
            }
            best = best.max(diff);
        }

        if best == i32::MIN { 0 } else { best }
    }
}