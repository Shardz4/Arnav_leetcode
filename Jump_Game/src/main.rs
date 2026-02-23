pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut max_reach: usize = 0;
    let len = nums.len();
    
    for i in 0..len {
        if i > max_reach {
            return false;
        }
        let jump = nums[i].max(0) as usize;
        max_reach = max_reach.max(i + jump);
        if max_reach >= len - 1 {
            return true;
        }
    }
    
    max_reach >= len - 1
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    let result = can_jump(nums);
    println!("Can jump: {}", result);

    let nums2 = vec![3, 2, 1, 0, 4];
    let result2 = can_jump(nums2);
    println!("Can jump: {}", result2);
}