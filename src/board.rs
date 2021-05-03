use crate::cell_group::CellGroup;

pub struct Board {
    rows: CellGroup,
    cols: CellGroup,
    boxes: CellGroup,
    pub board: Vec<Vec<i32>>,
}

impl Board {
    pub fn from_string(board_csv: &str) -> Board {
        Board {
            rows: CellGroup::new(),
            cols: CellGroup::new(),
            boxes: CellGroup::new(),
            board: vec![vec![0; 9]; 9]
        }
    }

    pub fn solve(&self) {
    }
}
