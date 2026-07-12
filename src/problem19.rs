fn canPlaceSecurityCameras(N: i32, grid: &[Vec<i32>]) -> bool {
    let n = N as usize;

    // N non-attacking cameras need N distinct rows and N distinct cols.
    if grid.len() < n || grid.iter().any(|row| row.len() < n) {
        return false;
    }

    let mut cols = vec![false; n];
    let mut diag_up = vec![false; 2 * n]; // index r + c
    let mut diag_down = vec![false; 2 * n]; // index r + (n-1) - c

    solve(0, n, grid, &mut cols, &mut diag_up, &mut diag_down)
}

fn solve(
    r: usize,
    n: usize,
    grid: &[Vec<i32>],
    cols: &mut [bool],
    diag_up: &mut [bool],
    diag_down: &mut [bool],
) -> bool {
    if r == n {
        return true; // placed one in every row → all n cameras down
    }

    for c in 0..n {
        if grid[r][c] == 1 {
            continue; // blocked cell
        }
        let up = r + c;
        let down = r + (n - 1) - c;
        if cols[c] || diag_up[up] || diag_down[down] {
            continue; // conflict on column or a diagonal
        }

        // place
        cols[c] = true;
        diag_up[up] = true;
        diag_down[down] = true;

        if solve(r + 1, n, grid, cols, diag_up, diag_down) {
            return true;
        }

        // undo (backtrack)
        cols[c] = false;
        diag_up[up] = false;
        diag_down[down] = false;
    }

    false // no column worked in this row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_4x4_solvable() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert!(canPlaceSecurityCameras(4, &grid));
    }

    #[test]
    fn blocked_4x4_still_solvable() {
        // Example 2 from the prompt
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
            vec![0, 0, 1, 0],
        ];
        assert!(canPlaceSecurityCameras(4, &grid));
    }

    #[test]
    fn single_cell_empty() {
        // Sample Input 0
        let grid = vec![vec![0]];
        assert!(canPlaceSecurityCameras(1, &grid));
    }

    #[test]
    fn single_cell_blocked() {
        let grid = vec![vec![1]];
        assert!(!canPlaceSecurityCameras(1, &grid));
    }

    #[test]
    fn n2_impossible() {
        // Sample Input 1 — two queens on 2x2 always attack diagonally
        let grid = vec![vec![0, 0], vec![0, 0]];
        assert!(!canPlaceSecurityCameras(2, &grid));
    }

    #[test]
    fn n3_impossible() {
        // Classic: no solution to 3-queens even on an empty board
        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert!(!canPlaceSecurityCameras(3, &grid));
    }

    #[test]
    fn grid_too_small_for_n() {
        // N=5 but grid is 3x4 — fewer rows/cols than cameras needed
        let grid = vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![1, 0, 1, 0],
        ];
        assert!(!canPlaceSecurityCameras(5, &grid));
    }

    #[test]
    fn blocking_forces_no_solution() {
        // Block the whole first row → row 0 can't place → false
        let grid = vec![
            vec![1, 1, 1, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert!(!canPlaceSecurityCameras(4, &grid));
    }

    #[test]
    fn n8_empty_solvable() {
        // 8-queens has solutions
        let grid = vec![vec![0; 8]; 8];
        assert!(canPlaceSecurityCameras(8, &grid));
    }

    #[test]
    fn forced_single_path() {
        // Only one legal column per row; verify it's found
        // Row0→c1, Row1→c3, Row2→c0, Row3→c2 (the arrangement from Example 1)
        let grid = vec![
            vec![1, 0, 1, 1],
            vec![1, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![1, 1, 0, 1],
        ];
        assert!(canPlaceSecurityCameras(4, &grid));
    }
}
