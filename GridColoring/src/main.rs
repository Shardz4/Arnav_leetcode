const MOD: i64 = 1_000_000_007;

fn is_valid_row(row: &Vec<i32>) -> bool {
    for i in 1..row.len() {
        if row[i] == row[i - 1] {
            return false;
        }
    }
    true
}

fn are_compatible(row1: &Vec<i32>, row2: &Vec<i32>) -> bool{
    for i in 0..row1.len() {
        if row1[i] == row2[i] {
            return false;
        }
    }
    true
}

fn generate_valid_rows(n: usize, colors: i32, curr: &mut Vec<i32>, rows: &mut Vec<Vec<i32>>){
    if curr.len() == n{
        if is_valid_row(curr) {
            rows.push(curr.clone());

        }
        return;
    }
    for i in 0..colors{
        curr.push(i);
        generate_valid_rows(n, colors, curr, rows);
        curr.pop();
    }
}

fn matrix_multiply(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>, modulus: i64) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut result = vec![vec![0; n]; n];

for i in 0..n{
    for j in 0..n{
        for k in 0..n{
            result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % modulus;
        }
    }
}
    result
}

fn matrix_exponentiation(mut mat: Vec<Vec<i64>>, mut exp: i32, modulus: i64) -> Vec<Vec<i64>> {
    let n = mat.len();
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        result[i][i] = 1;
    }

    while exp > 0 {
        if exp % 2 == 1 {
            result = matrix_multiply(&result, &mat, modulus);
        }
        mat = matrix_multiply(&mat, &mat, modulus);
        exp >>= 1;
    }
    result
}

fn color_the_grid(m: i32, n: i32) -> i32{
    let m = m as usize;
    let n = n as usize;
    let mut valid_rows: Vec<Vec<i32>> = vec![];
    let mut curr: Vec<i32> = vec![];
    generate_valid_rows(m, 3, &mut curr, &mut valid_rows);

    let len = valid_rows.len();
    let mut transition = vec![vec![0; len]; len];
    for i in 0..len {
        for j in 0..len {
            if are_compatible(&valid_rows[i], &valid_rows[j]) {
                transition[i][j] = 1;
            }
        }
    }

    let powered = matrix_exponentiation(transition, n as i32 - 1, MOD);

    let initial = vec![1; len];
    let mut result = vec![0; len];

    for i in 0..len{
        for j in 0..len {
            result[i] = (result[i] + powered[i][j] * initial[j]) % MOD;
        }
    }
    let mut total = 0;
    for i in 0..len {
        total = (total + result[i]) % MOD;
    }
    total as i32
}

fn main() {
    let m = 5;
    let n = 5;
    let result = color_the_grid(m, n);
    println!("Number of ways to color the grid: {}", result);
}



