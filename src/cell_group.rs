use std::collections::HashSet;

pub struct CellGroup {
    cells: HashSet<i8>,
}

impl CellGroup {
    pub fn new() -> CellGroup {
        CellGroup {
            cells: HashSet::with_capacity(9)
        }
    }

    pub fn insert(&self, digit: i8) {
        if digit < 0 || digit > 9 {
            panic!("Value must be 0 to 9 inclusive");
        }
    }

    pub fn remove(&mut self, digit: i8) {
        // self.cells.remove(&digit);
    }
}

#[test]
#[should_panic(expected="Value must be 0 to 9 inclusive")]
fn test_insert_rejects_values_less_than_zero() {
    let cg = CellGroup::new();

    cg.insert(-1);
}

#[test]
fn test_insert_accepts_0_to_9() {
    let cg = CellGroup::new();

    for i in 0..10 {
        cg.insert(i);
    }
}

#[test]
#[should_panic(expected="Value must be 0 to 9 inclusive")]
fn test_insert_rejects_values_more_than_nine() {
    let cg = CellGroup::new();

    cg.insert(10);
}

#[test]
fn test_insert_stores_digit() {
    let cg = CellGroup::new();

}

#[test]
fn test_contains_returns_true_if_number_is_stored() {
    let cg = CellGroup::new();

}

#[test]
fn test_contains_returns_false_if_number_is_not_stored() {
    let cg = CellGroup::new();

}
