pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut red = 0;
    let mut white = 0;
    let mut blue = nums.len() as i32 - 1;

    while white <= blue {
        match nums[white as usize] {
            0 => {
                nums.swap(red, white as usize);
                red += 1;
                white += 1;
            }
            1 => {
                white += 1;
            }
            2 => {
                nums.swap(white as usize, blue as usize);
                blue -= 1;
            }
            _ => panic!("Invalid color value"),
        }
    }
}

fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    println!("Sorted colors: {:?}", nums);
}