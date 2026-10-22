//! Wire the Python detector and its nearest tests.
//! Nối detector Python với test gần owner nhất.

#[path = "python.rs"]
mod python;

pub use python::python_step;

#[cfg(test)]
#[path = "test/python.rs"]
mod tests;
