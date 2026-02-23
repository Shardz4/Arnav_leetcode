pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let result = digits.rev();
    result[0] += 1;
    retrun result.rev().collect::<Vec<i32>>()

}