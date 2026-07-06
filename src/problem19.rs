use core::iter::Iterator;
use std::io::{self, BufRead};

/*
 * Place N Cameras Without Conflict on Blocked Grid

Given an NxN grid where 0 is empty and 1 is blocked, return true if N cameras can be placed on empty cells such that no two share the same row, column, or diagonal.
Examples
Example 1

Input:
N = 4
grid = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]

Output:
True

Explanation:
    We need to place 4 cameras so that no two share a row, column, or diagonal. One valid arrangement is:
        Row 0 → column 1
        Row 1 → column 3
        Row 2 → column 0
        Row 3 → column 2
    Each row has exactly one camera, no two are in the same column, and no two lie on the same diagonal. Thus the function returns true.

Example 2
Input:
N = 4
grid = [[0, 1, 0, 0], [0, 0, 0, 1], [1, 0, 0, 0], [0, 0, 1, 0]]

Output:
True

Explanation:
    Some cells are blocked by skylights (marked 1).
    We still need 4 cameras. One possible placement:
        Row 0 → column 2 (cell [0][2] is empty)
        Row 1 → column 0 (cell [1][0] is empty, no conflict with row 0)
        Row 2 → column 3 (cell [2][3] is empty, no conflicts)
        Row 3 → column 1 (cell [3][1] is empty, no conflicts)
    All cameras are on empty cells, and none share a row, column, or diagonal.
    The function returns true.

Input Format

    The first line contains a single integer, N.
    The second line contains a single integer, grid_rows, representing the number of rows in the grid.
    The third line contains a single integer, grid_columns, representing the number of columns in the grid.
    The next grid_rows lines describe the grid. Each of these lines contains grid_columns space-separated integers.

5
3
4
1 0 0 1
0 1 1 0
1 0 1 0

Explanation:

    N = 5
    grid_rows = 3
    grid_columns = 4
    grid = [[1, 0, 0, 1], [0, 1, 1, 0], [1, 0, 1, 0]]

Constraints

    1 <= N <= 15
    grid.length == N
    For all 0 <= i < N, grid[i].length == N
    For all 0 <= i, j < N, grid[i][j] ∈ {0, 1}

Output Format

Return a single boolean value (true or false) indicating whether a valid arrangement of N cameras exists under the given constraints.

Sample Input 0

1
1
1
0

Sample Output 0

1

Sample Input 1

2
2
2
0 1
1 0

Sample Output 1

0

 */

fn canPlaceSecurityCameras(N: i32, grid: &[Vec<i32>]) -> bool {
    // Write your code here
}

fn is_clear_horizontal(x: usize, row: Vec<i32>) -> bool {
    for i in 0..row.len() {
        //do not check self
        if i == x {
            continue;
        }
        if row[i] == -1 {
            return false;
        }
    }
    true
}

fn is_clear_vertical(x: usize, y: usize, grid: &[Vec<i32>]) -> bool {
    //iterate over every row
    for i in 0..grid.len() {
        //if it is the row of the security camera, skip
        if x == i {
            continue;
        }
        //check the column
        let curr_row = &grid[i];
        if curr_row[y] == -1 {
            return false;
        }
    }
    true
}

fn is_clear_diagonal(x: usize, y: usize, grid: &[Vec<i32>]) -> bool {
    //we start with x,y
    //we need to check:
    //( x+1, y+1 ), (x+2, y+2) up to and excluding x>num_cols
    for i in (x..grid[0].len()) {
        let row = &grid[i];
        for j in (y..grid.len()) {
            if row[j] == -1 {
                return false;
            }
        }
    }
    //, (x-1,y+1), (x-2,y+1) up to and excluding y>num_rows
    for i in (0..x).rev() {}
    //( x+1, y-1 ), (x+2, y-2) down to 0 and x<grid[i].len()
    for i in (x..grid.len()) {}
    //symmetrically ( x-1, y-1 ),(x-2, y-1) etc
    for i in (0..x).rev() {}
    //iterate over every row
    // for i in 0..grid.len() {
    //     //iterate over every column
    //     for j in grid[i].len() {}
    // }
    false
}
