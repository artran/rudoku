use std::collections::HashSet;

pub struct CellGroup {
    content: HashSet<i8>
}

impl CellGroup {
    pub fn new() -> CellGroup {
        CellGroup {
            content: HashSet::with_capacity(9)
        }
    }
}

//     def test_rejects_non_integer(self):
//         cell_group = CellGroup()
//         assert_that(cell_group.insert).raises(ValueError).when_called_with('x')
//
//     def test_cell_rejects_values_less_than_one(self):
//         cell_group = CellGroup()
//
//         for i in range(-5, 1):
//             with self.subTest('Candidate values must be greater than zero', candidate=i):
//                 try:
//                     cell_group.insert(i)
//                     fail(f'{i} is not acceptable')
//                 except ValueError:
//                     pass
//
//     def test_cell_rejects_values_more_than_nine(self):
//         cell_group = CellGroup()
//
//         for i in range(10, 15):
//             with self.subTest('Candidate values must be less than ten', candidate=i):
//                 try:
//                     cell_group.insert(i)
//                     fail(f'{i} is not acceptable')
//                 except ValueError:
//                     pass
//
//     def test_cell_accepts_integer_between_1_and_9(self):
//         cell_group = CellGroup()
//
//         for i in range(1, 10):
//             with self.subTest('Candidate values must be between 1 and 9 inclusive', candidate=i):
//                 cell_group.insert(i)
//
//     def test_insert_returns_true_if_number_is_possible(self):
//         cell_group = CellGroup()
//         for i in range(1, 9):
//             cell_group.insert(i)  # only 9 is missing
//
//         for i in range(1, 10):
//             with self.subTest('Only 9 should be possible', i=i):
//                 if i != 9:
//                     assert_that(cell_group.insert(i)).is_false()
//                 else:
//                     assert_that(cell_group.insert(i)).is_true()
