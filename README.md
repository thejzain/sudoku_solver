## Rusty Sudoku Solver

This repository contains a Rust program that can solve Sudoku puzzles.

### Features

* Solves Sudoku puzzles using backtracking algorithm.
* Checks for initial validity (no duplicates in rows, columns, or boxes).
* Handles 9x9 Sudoku puzzles.

### Usage

1. Clone the repository:

```bash
git clone https://github.com/<your-username>/rusty-sudoku-solver.git
```

2. Make sure you have Rust and Cargo installed ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)).

3. Modify the `sudoku` variable in the `main` function of `src/main.rs` to represent your desired Sudoku puzzle. 

   - A valid Sudoku board is a 9x9 grid where each row, column, and 3x3 subgrid (box) contains the numbers 1 to 9 without repetition.
   - Use `0` to represent empty cells.

   Here's an example with a modified board:

   ```rust
   fn main() {
       let mut sudoku: [[i8; 9]; 9] = [
           [0, 2, 0, 0, 0, 0, 0, 6, 8],
           [7, 0, 0, 0, 3, 6, 1, 0, 0],
           [0, 0, 0, 1, 2, 8, 0, 0, 5],
           [6, 0, 0, 0, 0, 1, 0, 7, 2],
           [0, 0, 5, 4, 0, 9, 3, 0, 0],
           [8, 0, 0, 0, 0, 3, 0, 0, 1],
           [0, 0, 3, 2, 1, 0, 0, 0, 0],
           [0, 0, 1, 6, 8, 0, 0, 9, 3],
           [4, 0, 0, 0, 0, 0, 0, 2, 0],
       ];
       // ... rest of the code ...
   }
   ```

4. Run the program:

```bash
cargo run
```

The program will attempt to solve your provided Sudoku puzzle and print the solution if found.

### How it works

The program utilizes several functions:

* `validity_check`: Ensures the initial Sudoku state is valid (no duplicates in rows, columns, or boxes).
* `solve`: Solves the Sudoku puzzle using backtracking. It iteratively tries different values for empty cells and checks for validity after each placement.
* `find_empty_cell`: Locates the first empty cell in the Sudoku.
