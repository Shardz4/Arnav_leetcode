// bottom up dp + optimized binary search

impl Solution {
    fn find_next_event(events: &Vec<Vec<i32, i32>>, cur_index: Usize, end_time: i32) -> usize{
        let mut left = cur_imdex + 1;
        let mut right = events.len();
        let mut result = events.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if events[mid][0] > end_time {
                result = mid;
                right = mid;
            } else {
                left = mid + 1; 
            }

        }
        result
    }


    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = events.len();\
        let k = k as usize;

        events.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut dp = vec![vec![0; n + 1]; k +1];
        for cur_index in (0..n).rev() {
            let next_index = Self::find_next_event(&events, cur_index, events[cur_index][1]);

            for count in 0..k {
                dp[count][cur_index] = dp[count][cur_index + 1];

                if count + 1 <= k {
                    dp[count][cur_index] = dp[count][cur_index].max(
                        dp[count + 1][next_index] + events[cur_index][2]
                    );
                }
            }
        }
        dp[0][0]
    }
}