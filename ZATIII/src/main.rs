use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        queries.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut heap = BinaryHeap::new();
        let mut delta_array = vec![0; nums.len() + 1];
        let mut operations = 0;
        let mut j = 0;
        
        for i in 0..nums.len() {
            operations += delta_array[i];
            while j < queries.len() && queries[j][0] == i as i32 {
                heap.push(queries[j][1]);
                j += 1;
            }
            while operations < nums[i] && !heap.is_empty() && *heap.peek().unwrap() >= i as i32 {
                operations += 1;
                let end = heap.pop().unwrap() as usize;
                delta_array[end + 1] -= 1;
            }
            if operations < nums[i] {
                return -1;
            }
        }
        heap.len() as i32
    }
}



// use std::{cmp::Reverse, collections::BinaryHeap};

// impl Solution {
//     pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
//         let mut result = queries.len() as i32;

//         queries.sort_unstable();

//         let mut queries_pq = BinaryHeap::new();
//         let mut query_i = 0;

//         let mut increment_pq = BinaryHeap::new();

//         let mut decrement = 0;

//         for (num_i, mut num) in nums.into_iter().enumerate() {
//             while let Some(&Reverse(i)) = increment_pq.peek() {
//                 if i > num_i {
//                     break;
//                 }

//                 decrement -= 1;
//                 increment_pq.pop();
//             }

//             num -= decrement;

//             while let Some(query) = queries.get(query_i) {
//                 if query[0] as usize > num_i {
//                     break;
//                 }

//                 queries_pq.push(query[1] as usize + 1);
//                 query_i += 1;
//             }

//             while num > 0 {
//                 increment_pq.push(Reverse(match queries_pq.pop() {
//                     Some(x) if x > num_i => x,
//                     _ => return -1,
//                 }));

//                 num -= 1;
//                 decrement += 1;
//                 result -= 1;
//             }
//         }

//         result
//     }
// }