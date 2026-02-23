use std::collections::{HashSet, VecDeque};
struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {

        let n = board.len() as i32; 
        let n2 = (n * n) as usize;  

   
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, 0)); 
        visited.insert(0);

        while let Some((curr, dist)) = queue.pop_front() {
            let curr_square = curr + 1; 
            if curr_square == n2 {
                return dist;
            }

            for roll in 1..=6 {
                let mut next = curr + roll; 
                
                if next >= n2 {
                    continue;
                }

                
                let next_square = (next + 1) as i32; 
                let row_from_bottom = (next_square - 1) / n; 
                let row = (n - 1 - row_from_bottom) as usize; 
                let col = if row_from_bottom % 2 == 0 {
                    
                    ((next_square - 1) % n) as usize
                } else {
                    
                    ((n - 1 - (next_square - 1) % n)) as usize
                };

                
                if board[row][col] != -1 {
                    next = (board[row][col] - 1) as usize; 
                }

                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, dist + 1));
                }
            }
        }
        -1
    }
}