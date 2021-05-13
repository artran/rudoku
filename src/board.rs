use csv;

use crate::cell_group::CellGroup;


pub struct Board {
    cols: Vec<CellGroup>,
    rows: Vec<CellGroup>,
    boxes: Vec<CellGroup>,
    pub board: Vec<Vec<i32>>,
}

impl Board {
    pub fn from_string(csv_str: &str) -> Self {
        let board = Self {
            cols: Self::initialise_empty_cell_groups(),
            rows: Self::initialise_empty_cell_groups(),
            boxes: Self::initialise_empty_cell_groups(),
            board: vec![vec![0; 9]; 9],
        };

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
        let board = Board::from_string("");

        assert_that!(board.cols).has_length(9);
    }

    #[test]
    fn test_new_board_has_nine_rows() {
        let board = Board::from_string("");

        assert_that!(board.rows).has_length(9);
    }

    #[test]
    fn test_new_board_has_nine_boxes() {
        let board = Board::from_string("");

        assert_that!(board.boxes).has_length(9);
    }
}
// def test_setting_value_puts_value_in_correct_groups(self):
//     test_data = [
//         # (col, row, box),
//         (3, 3, 1),
//         (5, 2, 2),
//         (7, 3, 3),
//         (2, 6, 4),
//         (6, 4, 5),
//         (8, 5, 6),
//         (2, 8, 7),
//         (4, 7, 8),
//         (9, 8, 9)
//     ]
//
//     for item in test_data:
//         with self.subTest(f'{item[0]}, {item[1]} -> {item[2]}', item=item):
//             board = Board()
//             board.set_value_at(item[0], item[1], 9)
//
//             with soft_assertions():
//                 assert_that(board.cols[item[0] - 1]).contains(9)
//                 assert_that(board.rows[item[1] - 1]).contains(9)
//                 assert_that(board.boxes[item[2] - 1]).contains(9)
//
// def test_setting_acceptable_value_returns_true(self):
//     board = Board()
//     assert_that(board.set_value_at(1, 1, 1)).is_true()
//
// def test_setting_unacceptable_value_returns_true(self):
//     board = Board()
//     board.set_value_at(1, 1, 1)
//     assert_that(board.set_value_at(2, 1, 1)).is_false()
//
// def test_setting_unacceptable_value_leaves_state_unchanged(self):
//     board = Board()
//     board.set_value_at(1, 1, 1)
//     initial_state = str(board)
//
//     board.set_value_at(2, 1, 1)
//     assert_that(str(board)).is_equal_to(initial_state)
//
// def test_setting_acceptable_value_updates_board(self):
//     expected_board = [
//         [1, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     ]
//     board = Board()
//
//     board.set_value_at(1, 1, 1)
//
//     assert_that(board.full_board).is_equal_to(expected_board)
//
// def test_setting_unacceptable_value_does_not_update_board(self):
//     expected_board = [
//         [1, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//         [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     ]
//     board = Board()
//
//     board.set_value_at(1, 1, 1)
//     board.set_value_at(2, 1, 1)
//
//     assert_that(board.full_board).is_equal_to(expected_board)
//
// def test_is_empty_at_return_true_if_cell_at_location_is_empty(self):
//     board = Board()
//
//     with soft_assertions():
//         assert_that(board.is_empty_at(1, 1)).is_true()
//
// def test_clear_value_at_removes_the_value_at_a_location(self):
//     board = Board()
//     board.set_value_at(1, 1, 1)
//
//     board.clear_value_at(1, 1)
//
//     with soft_assertions():
//         assert_that(board.cols[0]).does_not_contain(1)
//         assert_that(board.rows[0]).does_not_contain(1)
//         assert_that(board.boxes[0]).does_not_contain(1)
//         assert_that(board.full_board[0][0]).is_zero()
//
// def test_is_empty_at_return_false_if_cell_at_location_is_not_empty(self):
//     board = Board()
//     board.set_value_at(1, 1, 1)
//
//     with soft_assertions():
//         assert_that(board.is_empty_at(1, 1)).is_false()
//
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
