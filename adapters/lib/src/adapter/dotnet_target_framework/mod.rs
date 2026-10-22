//! Safe .NET target-framework selection from root csproj files.
//! Chọn target framework .NET an toàn từ csproj gốc.

mod dotnet_target_framework;
pub(crate) use dotnet_target_framework::read_dotnet_target_framework;
#[cfg(test)]
pub(crate) use dotnet_target_framework::read_dotnet_target_framework_file;
