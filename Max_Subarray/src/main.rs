pub fn max_subarray(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut curr_sum = nums[0];
    for &num in nums.iter().skip(1) {
        curr_sum = num.max(curr_sum + nums[i]);
        max_sum = max_sum.max(curr_sum);
    }
    max_sum
}
fn main() {
    let nums = vec![5, 4, -1, 7, 8];
    let result = max_subarray(nums);
    println!("Maximum subarray sum is: {}", result);
    
}
