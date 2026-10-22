//! Table model tests — kiểm thử model table.
//! Tests are children of the model module so private state stays private externally.
//! Test là child của model module nên state vẫn private với bên ngoài.

use super::Table;

#[test]
fn constructor_and_row_insertion_preserve_state_order() {
    let mut table = Table::new(vec!["Package".to_string(), "Version".to_string()]);
    assert_eq!(
        table.headers,
        vec!["Package".to_string(), "Version".to_string()]
    );
    assert!(table.rows.is_empty());

    table.add_row(vec!["alpha".to_string(), "1.0".to_string()]);
    table.add_row(vec!["beta".to_string(), "2.0".to_string()]);
    assert_eq!(table.rows.len(), 2);
    assert_eq!(table.rows[0], vec!["alpha".to_string(), "1.0".to_string()]);
    assert_eq!(table.rows[1], vec!["beta".to_string(), "2.0".to_string()]);
}

#[test]
fn public_methods_keep_their_original_signatures() {
    let _new: fn(Vec<String>) -> Table = Table::new;
    let _add_row: fn(&mut Table, Vec<String>) = Table::add_row;
    let _print: fn(&Table) = Table::print;
}
