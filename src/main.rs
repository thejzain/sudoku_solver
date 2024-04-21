use std::collections::HashMap;
use std::string::ParseError;

fn main() {
    let mut sudoku: [[i8; 9]; 9] = [
        [0, 1, 0, 0, 7, 0, 0, 5, 0],
        [8, 0, 0, 1, 0, 0, 7, 4, 3],
        [4, 0, 0, 0, 3, 0, 2, 0, 0],
        [7, 0, 0, 0, 1, 3, 0, 0, 6],
        [0, 0, 1, 6, 0, 0, 0, 0, 0],
        [3, 6, 8, 0, 0, 4, 0, 7, 9],
        [0, 8, 7, 3, 0, 5, 9, 6, 0],
        [9, 3, 0, 0, 2, 0, 8, 0, 0],
        [0, 0, 5, 8, 9, 0, 4, 0, 7],
    ];
    println!("{:?}", validity_check(&sudoku));
}

fn validity_check_unfin(sudoku: &[[i8; 9]; 9]) -> Result<(), String> {
    let mut check: HashMap<i8, bool> = HashMap::new();

    //Checks all row for duplicates
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j] == 0 {
                continue;
            }
            match check.get(&sudoku[i][j]) {
                Some(_) => {
                    return Err(format!("Row check {}, {}", i, j));
                }
                None => {
                    check.insert(sudoku[i][j], true);
                }
            }
        }
        check.clear();
    }

    //Checks all column for duplicates
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[j][i] == 0 {
                continue;
            }
            match check.get(&sudoku[j][i]) {
                Some(_) => {
                    return Err(format!("Col check {}, {}", i, j));
                }
                None => {
                    check.insert(sudoku[j][i], true);
                }
            }
        }
        check.clear();
    }

    //Check the 3x3 boxes
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j] == 0 {
                continue;
            }
        }
    }
    Ok(())
}

fn validity_check(sudoku: &[[i8; 9]; 9]) -> Result<(), String> {
    let mut check: HashMap<String, i8> = HashMap::new();
    for i in 0..9 {
        for j in 0..9 {
            if sudoku[i][j] == 0 {
                continue;
            }
            let row_key = format!("r{}{}", i, sudoku[i][j]);
            let col_key = format!("c{}{}", j, sudoku[i][j]);
            let box_key = format!("b{}{}{}", i / 3, j / 3, sudoku[i][j]);

            if let Some(_) = check.insert(row_key, sudoku[i][j]) {
                return Err(format!("Duplicate value in row {}", i));
            }
            if let Some(_) = check.insert(col_key, sudoku[i][j]) {
                return Err(format!("Duplicate value in column {}", j));
            }
            if let Some(_) = check.insert(box_key, sudoku[i][j]) {
                return Err(format!("Duplicate value in box {},{}", i / 3, j / 3));
            }
        }
    }
    Ok(())
}
