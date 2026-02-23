use std::collections::HashMap;

impl Solution {
    fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut freq_map: HashMap<char, i32> = HashMap::new();

        for c in word.chars() {
            *freq_map.entry(c).or_insert(0) += 1;
        }

        let mut freqs: Vec<i32> = freq_map.values().cloned().collect();
        freqs.sort_unstable();
        let unique_freqs: Vec<i32> = freqs.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();

        let mut min_deletions = i32::MAX;

        for &target in &unique_freqs {
            let mut deletions = 0;

            for &freq in freq_map.values() {
                if freq < target {
                    deletions += freq;
                } else if freq > target + k {
                    deletions += freq - (target + k);
                }
            }
            min_deletions = std::cmp::min(min_deletions, deletions);
        }

        if min_deletions == i32::MAX {
            -1
        } else {
            min_deletions
        }
    }
}