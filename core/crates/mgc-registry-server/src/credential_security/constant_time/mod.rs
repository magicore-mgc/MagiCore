//! Wires the constant-time comparator and its nearest tests.
//! Nối bộ so sánh hằng thời gian và test gần nhất.

mod constant_time;
pub(crate) use constant_time::constant_time_eq;
#[cfg(test)]
mod test;
