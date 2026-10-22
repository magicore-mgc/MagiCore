//! dotnet recognizer wiring.
//! Wiring bộ nhận diện dotnet.
mod dotnet;
pub(in crate::project::project::core_detection) use dotnet::detect_dotnet_cores;
