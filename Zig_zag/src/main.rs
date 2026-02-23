pub fn convert (s: String, num_rows: i32) -> String {

    if num_rows == 1 || num_rows >= s.len() as i32 {
        return s;
    }

    let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
    let mut current_row = 0;
    let mut step: i32 = 1;

    for c in s.chars() {
        // Append character to the current row
        rows[current_row].push(c);
        if current_row ==0 {
            step = 1; // Move down
        }
        else if current_row == (num_rows -1) as usize {
            step = -1;
        }
        current_row = (current_row as i32 + step) as usize;

    }

    rows.into_iter().collect()
}

fn main() {
    let s = String::from("PAYPALISHIRING");
    let num_rows = 3;
    let result = convert(s, num_rows);
    println!("Converted string: {}", result);
}