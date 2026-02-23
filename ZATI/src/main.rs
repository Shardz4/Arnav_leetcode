
// Difference array implementation

pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool{
    let n = nums.len();

    // counting how many queries cover each index

    let mut query_count = vec![0; n+1];

    for q in &queries{
        let li = q[0] as usize;
        let ri = q[1] as usize;
        query_count[li] += 1;
        if ri+1 < n{
            query_count[ri + 1] -= 1;
        }
    }

    for i in 1..n{
        query_count[i] += query_count[i-1];
    }

    for i in 0..n {
        if query_count[i] < nums [i] {
            return false;
        }
    }

    let mut decr = vec![0; n+1];
    for i in 0..n {
        decr[i] += nums[i];
        if i + 1 < n {
            decr [i+1] -= nums[i];
        }

    }
    for i in 0..n {
         decr[i] += decr[i-1]
        }

        for q in queries {
            let li = q[0] as usize;
            let ri = q[1] as usize;
            let mut max_decr = 0;
            for j in li..ri {
                max_decr = max_decr.max(decr[j]);
            }

            if max_decr > (ri - li + 1) as i32 {
                return false;
            }
        }
    true
    }

    fn main() {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
        let result = is_zero_array(nums, queries);
        println!("Is zero array: {}", result);
    }
