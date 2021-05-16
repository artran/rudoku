use csv;

use crate::cell_group::CellGroup;


pub struct Board {
    cols: Vec<CellGroup>,
    rows: Vec<CellGroup>,
    squares: Vec<CellGroup>,
    pub board: Vec<Vec<u8>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cols: Self::initialise_empty_cell_groups(),
            rows: Self::initialise_empty_cell_groups(),
            squares: Self::initialise_empty_cell_groups(),
            board: vec![vec![0; 9]; 9],
        }
    }

    pub fn from_string(csv_str: &str) -> Self {
        let board = Self::new();

        board.fill_from_csv(csv_str);

        board
    }

    fn initialise_empty_cell_groups() -> Vec<CellGroup> {
        let mut groups = Vec::new();

        for _ in 0..9 {
            groups.push(CellGroup::new());
        };

        groups
    }

    fn fill_from_csv(&self, csv_str: &str) {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(csv_str.as_bytes());

        for result in rdr.records() {
            let record = result.unwrap();
            println!("{:?}", record);
        };
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

    fn is_empty_at(&self, col_idx: &usize, rox_idx: &usize) -> bool {
        self.board[rox_idx - 1][col_idx - 1] == 0
    }

    pub fn solve(&self) {}
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

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
}

// def test_the_board_can_print_itself(self):
//     expected_output = '[[1, 0, 0, 0, 0, 0, 0, 0, 0],\n' \
//                       ' [0, 0, 0, 0, 0, 0, 0, 0, 0],\n' \
//                       ' [0, 0, 0, 0, 0, 0, 0, 0, 0],\n' \
//                       ' [0, 0, 0, 0, 0, 0, 0, 0, 0],\n' \
//                       ' [0, 0, 0, 0, 0, 0, 0, 0, 0],\n' \
//                       ' [0, 0, 0, 0, 0, 0, 0, 0, 0],\n' \
//                       ' [0, 0, 0, 0, 0, 0, 0, 0, 0],\n' \
//                       ' [0, 0, 0, 0, 0, 0, 0, 0, 0],\n' \
//                       ' [0, 0, 0, 0, 0, 0, 0, 0, 0]]\n'
//     output_stream = StringIO()
//     board = Board()
//     board.set_value_at(1, 1, 1)
//     board.print(output_stream)
//
//     assert_that(output_stream.getvalue()).is_equal_to(expected_output)
//
// def test_the_board_can_be_initialised_from_csv_file(self):
//     board_file = StringIO('6,,,,,3,,,\n'
//                           ',9,,8,6,7,3,,\n'
//                           ',3,7,2,5,,6,,\n'
//                           ',,4,1,3,,2,,\n'
//                           '3,,9,,,,1,,5\n'
//                           ',,8,,2,9,4,,\n'
//                           ',,5,,4,1,7,2,\n'
//                           ',,3,7,9,6,,8,\n'
//                           ',,,5,,,,,1\n')
//     expected_board = [
//         [6, 0, 0, 0, 0, 3, 0, 0, 0],
//         [0, 9, 0, 8, 6, 7, 3, 0, 0],
//         [0, 3, 7, 2, 5, 0, 6, 0, 0],
//         [0, 0, 4, 1, 3, 0, 2, 0, 0],
//         [3, 0, 9, 0, 0, 0, 1, 0, 5],
//         [0, 0, 8, 0, 2, 9, 4, 0, 0],
//         [0, 0, 5, 0, 4, 1, 7, 2, 0],
//         [0, 0, 3, 7, 9, 6, 0, 8, 0],
//         [0, 0, 0, 5, 0, 0, 0, 0, 1]
//     ]
//
//     board = Board()
//     board.load_file(board_file)
//
//     assert_that(board.full_board).is_equal_to(expected_board)
//
// @unittest.expectedFailure
// def test_that_the_board_solves_sudoku(self):
//     board_file = StringIO('6,,,,,3,,,\n'
//                           ',9,,8,6,7,3,,\n'
//                           ',3,7,2,5,,6,,\n'
//                           ',,4,1,3,,2,,\n'
//                           '3,,9,,,,1,,5\n'
//                           ',,8,,2,9,4,,\n'
//                           ',,5,,4,1,7,2,\n'
//                           ',,3,7,9,6,,8,\n'
//                           ',,,5,,,,,1\n')
//     expected_board = [
//         [6, 4, 2, 9, 1, 3, 8, 5, 7],
//         [5, 9, 1, 8, 6, 7, 3, 4, 2],
//         [8, 3, 7, 2, 5, 4, 6, 1, 9],
//         [7, 6, 4, 1, 3, 5, 2, 9, 8],
//         [3, 2, 9, 4, 7, 8, 1, 6, 5],
//         [1, 5, 8, 6, 2, 9, 4, 7, 3],
//         [9, 8, 5, 3, 4, 1, 7, 2, 6],
//         [2, 1, 3, 7, 9, 6, 5, 8, 4],
//         [4, 7, 6, 5, 8, 2, 9, 3, 1]
//     ]
//
//     board = Board()
//     board.load_file(board_file)
//     board.solve()
//
//     assert_that(board.full_board).is_equal_to(expected_board)
