//! Table model and API — model và API table.
//! State mutation remains local and printing delegates to the render owner.
//! Mutation state ở lại owner này; thao tác in giao cho render owner.

use super::super::render::print_table;

/// Simple table for displaying data — bảng đơn giản để hiển thị dữ liệu.
/// Fields stay private — các field tiếp tục private.
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    /// Create a table with the given headers — tạo bảng với header đã cho.
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
        }
    }

    /// Append a row without changing insertion order — thêm row, giữ thứ tự chèn.
    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    /// Print the table to stdout — in bảng ra stdout.
    /// Header, separator and rows keep their original order — giữ thứ tự header/separator/row cũ.
    pub fn print(&self) {
        print_table(&self.headers, &self.rows);
    }
}

#[cfg(test)]
#[path = "test/table.rs"]
mod tests;
