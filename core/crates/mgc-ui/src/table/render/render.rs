//! Table line formatting and output — format dòng table và output.
//! The production writer uses the same pure line helpers covered by tests.
//! Writer production dùng cùng helper dòng thuần được test bao phủ.

use console::style;

pub(super) fn header_line(headers: &[String]) -> String {
    headers
        .iter()
        .map(|header| style(header).bold().to_string())
        .collect::<Vec<_>>()
        .join("  ")
}

pub(super) fn separator_line() -> String {
    "─".repeat(80)
}

pub(super) fn row_line(row: &[String]) -> String {
    row.join("  ")
}

pub(in crate::table) fn print_table(headers: &[String], rows: &[Vec<String>]) {
    println!("{}", header_line(headers));
    println!("{}", separator_line());
    for row in rows {
        println!("{}", row_line(row));
    }
}
