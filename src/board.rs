use itertools::join;

use crate::cell_group::CellGroup;

pub struct Board {
    cols: Vec<CellGroup>,
    rows: Vec<CellGroup>,
    squares: Vec<CellGroup>,
    pub board: Vec<Vec<u8>>,
    pub solved_board: Vec<Vec<u8>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cols: Self::initialise_empty_cell_groups(),
            rows: Self::initialise_empty_cell_groups(),
            squares: Self::initialise_empty_cell_groups(),
            board: vec![vec![0; 9]; 9],
            solved_board: vec![vec![0; 9]; 9],
        }
    }

    pub fn from_string(board_str: &str) -> Self {
        // Fixme: This is incorrect as it doesn't initialise the CellGroups
        let mut board = Self::new();

        let mut board_iterator = board_str.chars().map(|c| c.to_digit(10).unwrap() as u8);
        for row_idx in 1..10 {
            for col_idx in 1..10 {
                board.set_value_at(&col_idx, &row_idx, board_iterator.next().unwrap());
            }
        }

        board
    }

    fn initialise_empty_cell_groups() -> Vec<CellGroup> {
        let mut groups = Vec::new();

        for _ in 0..9 {
            groups.push(CellGroup::new());
        };

        groups
    }

    fn set_value_at(&mut self, col_idx: &usize, row_idx: &usize, value: u8) -> bool {
        let col = &mut self.cols[col_idx - 1];
        let row = &mut self.rows[row_idx - 1];
        let square = &mut self.squares[(col_idx - 1) / 3 + 3 * ((row_idx - 1) / 3)];

        if col.contains(&value) || row.contains(&value) || square.contains(&value) {
            return false;
        }

        col.insert(value);
        row.insert(value);
        square.insert(value);
        self.board[row_idx - 1][col_idx - 1] = value;

        true
    }

    fn clear_value_at(&mut self, col_idx: &usize, row_idx: &usize) {
        let value = self.board[row_idx - 1][col_idx - 1];

        &mut self.cols[col_idx - 1].remove(&value);
        &mut self.rows[row_idx - 1].remove(&value);
        &mut self.squares[(col_idx - 1) / 3 + 3 * ((row_idx - 1) / 3)].remove(&value);
        self.board[row_idx - 1][col_idx - 1] = 0;
    }

    fn is_empty_at(&self, col_idx: &usize, row_idx: &usize) -> bool {
        self.board[row_idx - 1][col_idx - 1] == 0
    }

    pub fn print(&self) -> String {
        let mut result = String::new();

        for row in self.solved_board.iter() {
            result.push('[');
            result.push_str(&join(row, ", "));
            result.push_str("],\n");
        }

        result
    }

    pub fn solve(&mut self) {
        for row_idx in 1..10 {
            for col_idx in 1..10 {
                if !self.is_empty_at(&col_idx, &row_idx) {
                    continue;
                }

                for value in 1..10 {
                    if self.set_value_at(&col_idx, &row_idx, value) {
                        self.solve();
                        self.clear_value_at(&col_idx, &row_idx);
                    }
                }
                return;
            }
        }
        // Save the state before the recursion unwinds and resets the `full_board`
        self.solved_board = self.board.clone();
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use spectral::prelude::*;

    use crate::board::Board;

    #[test]
    fn test_new_board_has_nine_cols() {
        let board = Board::new();

        assert_that!(board.cols).has_length(9);
    }

    #[test]
    fn test_new_board_has_nine_rows() {
        let board = Board::new();

        assert_that!(board.rows).has_length(9);
    }

    #[test]
    fn test_new_board_has_nine_boxes() {
        let board = Board::new();

        assert_that!(board.squares).has_length(9);
    }

    #[test]
    fn test_setting_value_puts_value_in_correct_groups() {
        let test_data = [
            // (col, row, square),
            (3, 3, 1),
            (5, 2, 2),
            (7, 3, 3),
            (2, 6, 4),
            (6, 4, 5),
            (8, 5, 6),
            (2, 8, 7),
            (4, 7, 8),
            (9, 8, 9)
        ];

        for item in test_data.iter() {
            let mut board = Board::new();

            let col_idx = item.0;
            let row_idx = item.1;
            let box_idx = item.2;
            board.set_value_at(&col_idx, &row_idx, 9);

            assert_that!(&board.cols[(col_idx - 1)].contains(&9)).is_true();
            assert_that!(&board.rows[(row_idx - 1)].contains(&9)).is_true();
            assert_that!(&board.squares[(box_idx - 1)].contains(&9)).is_true();
            assert_that!(board.is_empty_at(&col_idx, &row_idx)).is_false();
        }
    }

    #[test]
    fn test_setting_acceptable_value_returns_true() {
        let mut board = Board::new();

        assert_that!(board.set_value_at(&1, &1, 1)).is_true();
    }

    #[test]
    fn test_setting_unacceptable_value_returns_false() {
        let mut board = Board::new();
        board.set_value_at(&1, &1, 1);

        assert_that!(board.set_value_at(&1, &1, 1)).is_false();
    }

    #[test]
    fn test_setting_unacceptable_value_leaves_state_unchanged() {
        let mut board = Board::new();
        board.set_value_at(&1, &1, 1);

        board.set_value_at(&2, &1, 1);

        assert_that!(&board.cols[1].contains(&1)).is_false();
    }

    #[test]
    fn test_setting_acceptable_value_updates_board() {
        let expected_board: Vec<Vec<u8>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let mut board = Board::new();

        board.set_value_at(&1, &2, 1);

        assert_that!(board.board).is_equal_to(expected_board);
    }

    #[test]
    fn test_setting_unacceptable_value_does_not_update_board() {
        let expected_board: Vec<Vec<u8>> = vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let mut board = Board::new();

        board.set_value_at(&1, &1, 1);
        board.set_value_at(&2, &1, 1);

        assert_that!(board.board).is_equal_to(expected_board);
    }

    #[test]
    fn test_is_empty_at_return_true_if_cell_at_location_is_empty() {
        let board = Board::new();

        assert_that!(board.is_empty_at(&1, &1)).is_true();
    }

    #[test]
    fn test_is_empty_at_return_false_if_cell_at_location_is_not_empty() {
        let mut board = Board::new();
        board.set_value_at(&1, &1, 1);

        assert_that!(board.is_empty_at(&1, &1)).is_false();
    }

    #[test]
    fn test_clear_value_at_removes_the_value_at_a_locations_col() {
        let mut board = Board::new();
        board.set_value_at(&1, &1, 1);

        board.clear_value_at(&1, &1);

        assert_that!(&board.cols[0].contains(&1)).is_false();
    }

    #[test]
    fn test_clear_value_at_removes_the_value_at_a_locations_row() {
        let mut board = Board::new();
        board.set_value_at(&1, &1, 1);
        board.clear_value_at(&1, &1);

        assert_that!(&board.rows[0].contains(&1)).is_false();
    }

    #[test]
    fn test_clear_value_at_removes_the_value_at_a_locations_square() {
        let mut board = Board::new();
        board.set_value_at(&1, &1, 1);
        board.clear_value_at(&1, &1);

        assert_that!(&board.squares[0].contains(&1)).is_false();
    }

    #[test]
    fn test_clear_value_at_removes_the_value_at_a_locations_cell() {
        let mut board = Board::new();
        board.set_value_at(&1, &1, 1);
        board.clear_value_at(&1, &1);

        assert_that!(&board.is_empty_at(&1, &1)).is_true();
    }

    #[test]
    fn test_the_board_can_print_itself() {
        let expected_output = String::from("[1, 2, 3, 0, 0, 0, 0, 0, 0],\n\
            [0, 0, 0, 0, 0, 0, 0, 0, 0],\n\
            [0, 0, 0, 0, 0, 0, 0, 0, 0],\n\
            [0, 0, 0, 0, 0, 0, 0, 0, 0],\n\
            [0, 0, 0, 0, 0, 0, 0, 0, 0],\n\
            [0, 0, 0, 0, 0, 0, 0, 0, 0],\n\
            [0, 0, 0, 0, 0, 0, 0, 0, 0],\n\
            [0, 0, 0, 0, 0, 0, 0, 0, 0],\n\
            [0, 0, 0, 0, 0, 0, 0, 0, 0],\n");
        let mut board = Board::new();
        board.set_value_at(&1, &1, 1);
        board.set_value_at(&2, &1, 2);
        board.set_value_at(&3, &1, 3);
        board.solved_board = board.board.clone();
        board.clear_value_at(&1, &1);
        board.clear_value_at(&2, &1);
        board.clear_value_at(&3, &1);

        let printed = board.print();

        assert_that!(printed).is_equal_to(expected_output);
    }

    #[test]
    fn test_the_board_can_be_initialised_from_a_string() {
        let board_file = String::from("600003000\
            090867300\
            037250600\
            004130200\
            309000105\
            008029400\
            005041720\
            003796080\
            000500001");

        let expected_board = vec![
            vec![6, 0, 0, 0, 0, 3, 0, 0, 0],
            vec![0, 9, 0, 8, 6, 7, 3, 0, 0],
            vec![0, 3, 7, 2, 5, 0, 6, 0, 0],
            vec![0, 0, 4, 1, 3, 0, 2, 0, 0],
            vec![3, 0, 9, 0, 0, 0, 1, 0, 5],
            vec![0, 0, 8, 0, 2, 9, 4, 0, 0],
            vec![0, 0, 5, 0, 4, 1, 7, 2, 0],
            vec![0, 0, 3, 7, 9, 6, 0, 8, 0],
            vec![0, 0, 0, 5, 0, 0, 0, 0, 1],
        ];
        let board = Board::from_string(&board_file);

        assert_that!(board.board).is_equal_to(expected_board);
    }

    #[test]
    fn test_that_the_board_solves_sudoku() {
        let board_file = String::from("600003000\
            090867300\
            037250600\
            004130200\
            309000105\
            008029400\
            005041720\
            003796080\
            000500001");
        let expected_solution = vec![
            vec![6, 4, 2, 9, 1, 3, 8, 5, 7],
            vec![5, 9, 1, 8, 6, 7, 3, 4, 2],
            vec![8, 3, 7, 2, 5, 4, 6, 1, 9],
            vec![7, 6, 4, 1, 3, 5, 2, 9, 8],
            vec![3, 2, 9, 4, 7, 8, 1, 6, 5],
            vec![1, 5, 8, 6, 2, 9, 4, 7, 3],
            vec![9, 8, 5, 3, 4, 1, 7, 2, 6],
            vec![2, 1, 3, 7, 9, 6, 5, 8, 4],
            vec![4, 7, 6, 5, 8, 2, 9, 3, 1]
        ];
        let mut board = Board::from_string(&board_file);

        board.solve();

        assert_that!(board.solved_board).is_equal_to(expected_solution);
    }
}
