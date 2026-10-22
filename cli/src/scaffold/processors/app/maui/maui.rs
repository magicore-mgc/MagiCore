//! MAUI implementation.
//! Triển khai MAUI.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::app::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    let slug = slugify(name);
    write_file(
        &target.join(format!("{slug}.csproj")),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <TargetFrameworks>net8.0-android;net8.0-ios;net8.0-maccatalyst</TargetFrameworks>\n    <OutputType>Exe</OutputType>\n    <RootNamespace>MagiCoreApp</RootNamespace>\n    <UseMaui>true</UseMaui>\n    <SingleProject>true</SingleProject>\n    <ImplicitUsings>enable</ImplicitUsings>\n    <Nullable>enable</Nullable>\n  </PropertyGroup>\n</Project>\n",
    )?;
    write_file(
        &target.join("MauiProgram.cs"),
        "using Microsoft.Maui;\nusing Microsoft.Maui.Hosting;\n\nnamespace MagiCoreApp;\n\npublic static class MauiProgram\n{\n    public static MauiApp CreateMauiApp() => MauiApp.CreateBuilder()\n        .UseMauiApp<App>()\n        .Build();\n}\n",
    )?;
    write_file(
        &target.join("App.xaml"),
        "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<Application xmlns=\"http://schemas.microsoft.com/dotnet/2021/maui\"\n             xmlns:x=\"http://schemas.microsoft.com/winfx/2009/xaml\"\n             x:Class=\"MagiCoreApp.App\">\n  <Application.Resources />\n</Application>\n",
    )?;
    write_file(
        &target.join("App.xaml.cs"),
        "namespace MagiCoreApp;\n\npublic partial class App : Application\n{\n    public App()\n    {\n        InitializeComponent();\n        MainPage = new MainPage();\n    }\n}\n",
    )?;
    write_file(
        &target.join("MainPage.xaml"),
        "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n<ContentPage xmlns=\"http://schemas.microsoft.com/dotnet/2021/maui\"\n             xmlns:x=\"http://schemas.microsoft.com/winfx/2009/xaml\"\n             x:Class=\"MagiCoreApp.MainPage\">\n  <VerticalStackLayout Padding=\"24\">\n    <Label Text=\"MagiCore MAUI app\" FontSize=\"24\" />\n  </VerticalStackLayout>\n</ContentPage>\n",
    )?;
    write_file(
        &target.join("MainPage.xaml.cs"),
        "namespace MagiCoreApp;\n\npublic partial class MainPage : ContentPage\n{\n    public MainPage() => InitializeComponent();\n}\n",
    )?;
    Ok(())
}
