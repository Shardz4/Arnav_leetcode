use std::collections::VecDeque;

    pub fn max_candies(status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 {
        let n = status.len();
        let mut total = 0;
        let mut queue: VecDeque<i32> = VecDeque::from(initial_boxes);
        let mut visited = vec![false; n];

        while let Some(box_id) = queue.pop_front(){
            if visited[box_id as usize] {
                continue;
            }
            visited[box_id as usize] = true;

            if status[box_id as usize] ==1 {
                total += candies[box_id as usize];

                for &key in &keys[box_id as usize] {
                    if !visited[key as usize] {
                        queue.push_back(key);
                    }
                }
                for &contained in &contained_boxes[box_id as usize] {
                    if !visited[contained as usize] {
                        queue.push_back(contained);
                    }
                }
            }
        }
        total
    }