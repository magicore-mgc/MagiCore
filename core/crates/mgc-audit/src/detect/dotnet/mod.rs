//! Wire the .NET detector and its nearest tests.
//! Nối detector .NET với test gần owner nhất.

#[path = "dotnet.rs"]
mod dotnet;

pub use dotnet::dotnet_step;

#[cfg(test)]
#[path = "test/dotnet.rs"]
mod tests;
