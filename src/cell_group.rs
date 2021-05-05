use std::collections::HashSet;

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

pub struct CellGroup {
    cells: HashSet<u8>,
}

impl CellGroup {
    pub fn new() -> CellGroup {
        CellGroup {
            cells: HashSet::with_capacity(9)
        }
    }

    pub fn insert(&mut self, digit: u8) {
        if digit > 9 {
            panic!("Value must be 0 to 9 inclusive");
        }

        self.cells.insert(digit);
    }

    pub fn contains(&self, digit: u8) -> bool {
        self.cells.contains(&digit)
    }

    pub fn remove(&mut self, digit: u8) {
        self.cells.remove(&digit);
    }
}

/* -------------------------------------------------------------------------------------------------
Tests
------------------------------------------------------------------------------------------------- */

#[test]
fn test_insert_accepts_0_to_9() {
    let mut cg = CellGroup::new();

    for i in 0..10 {
        cg.insert(i);
    }
}

#[test]
#[should_panic(expected="Value must be 0 to 9 inclusive")]
fn test_insert_rejects_values_more_than_nine() {
    let mut cg = CellGroup::new();

    cg.insert(10);
}

#[test]
fn test_insert_stores_digit() {
    let mut cg = CellGroup::new();
    let test_val: u8 = 5;
    assert_eq!(cg.cells.contains(&test_val), false);

    cg.insert(test_val);

    assert_eq!(cg.cells.contains(&test_val), true);
}

#[test]
fn test_contains_returns_true_if_number_is_stored() {
    let mut cg = CellGroup::new();
    cg.insert(5);

    assert_eq!(cg.contains(5), true);
}

#[test]
fn test_contains_returns_false_if_number_is_not_stored() {
    let cg = CellGroup::new();

    assert_eq!(cg.contains(5), false);
}

#[test]
fn test_remove_takes_digit_out_of_cell_group() {
    let mut cg = CellGroup::new();
    cg.insert(5);
    assert_eq!(cg.contains(5), true);

    cg.remove(5);

    assert_eq!(cg.contains(5), false);
}
