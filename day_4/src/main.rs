use std::fs;

fn main() {
    let input_data = fs::read_to_string("inputs/input_1.txt").expect("Could not read input file");

    let rows: Vec<String> = input_data.lines().map(|s| s.to_string()).collect();

    let horizontal_count = count_horizontal(&rows);

    let rows_transposed = transpose(&rows);
    let vertical_count = count_horizontal(&rows_transposed);

    let (diag_down_right, diag_down_left) = extract_diagonals(&rows);

    let diag_right_count = count_horizontal(&diag_down_right);
    let diag_left_count = count_horizontal(&diag_down_left);

    println!("The answer to task 1 is {}", horizontal_count + vertical_count + diag_right_count + diag_left_count);


}


fn count_horizontal(rows: &Vec<String>) -> usize {
    rows
        .into_iter()
        .map(|row| {
            row.as_bytes()
                .windows(4)
                .filter(|&w| w == b"XMAS" || w == b"SAMX")
                .count()
        })
        .sum()
}

fn transpose(rows: &Vec<String>) -> Vec<String> {
    let size = rows.len();
    let mut transposed = vec![String::with_capacity(size); size];

    for row in rows {
        for (col_index, ch) in row.chars().enumerate() {
            if transposed[col_index].is_empty() {
                transposed[col_index] = String::with_capacity(size);
            }
            transposed[col_index].push(ch);
        }
    }
    transposed
}
fn extract_diagonals(rows: &[String]) -> (Vec<String>, Vec<String>) {
    let r = rows.len();
    if r == 0 {
        return (vec![], vec![]);
    }
    let c = rows[0].len();
    let grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();

    let mut diag_down_right = Vec::new(); // top-left to bottom-right diagonals
    let mut diag_down_left = Vec::new();  // top-right to bottom-left diagonals

    // top-left to bottom-right diagonals
    // start from first row, all columns
    for start_col in 0..c {
        let mut d = String::new();
        let (mut x, mut y) = (0, start_col);
        while x < r && y < c {
            d.push(grid[x][y]);
            x += 1;
            y += 1;
        }
        if d.len() >= 4 {
            diag_down_right.push(d);
        }
    }
    // start from first column (excluding the top-left corner already covered), all rows
    for start_row in 1..r {
        let mut d = String::new();
        let (mut x, mut y) = (start_row, 0);
        while x < r && y < c {
            d.push(grid[x][y]);
            x += 1;
            y += 1;
        }
        if d.len() >= 4 {
            diag_down_right.push(d);
        }
    }

    // top-right to bottom-left diagonals
    // start from first row, all columns
    for start_col in 0..c {
        let mut d = String::new();
        let (mut x, mut y) = (0, start_col);
        while x < r && y < c {
            d.push(grid[x][y]);
            x += 1;
            if y == 0 { break; }
            y -= 1;            
        }
        if d.len() >= 4 {
            diag_down_left.push(d);
        }
    }
    // start from last column (excluding top-right corner)
    for start_row in 1..r {
        let mut d = String::new();
        let (mut x, mut y) = (start_row, c - 1);
        while x < r && y < c {
            d.push(grid[x][y]);
            x += 1;
            if y == 0 { break; }
            y -= 1; 
        }
        if d.len() >= 4 {
            diag_down_left.push(d);
        }
    }

    (diag_down_right, diag_down_left)
}
