//! Go Web backend starters (Echo, Fiber, Gin).
//! Starter backend Web Go (Echo, Fiber, Gin).

use std::path::Path;

use anyhow::{Result, bail};

use super::super::super::write_file;
use super::super::common::package_name;

pub(crate) fn write_go_backend(target: &Path, name: &str, framework: &str) -> Result<()> {
    let module_name = format!("example.com/{}", package_name(name));
    let (module, version, source) = match framework {
        "echo" => (
            "github.com/labstack/echo/v5",
            "v5.4.0",
            "package main\n\nimport (\n\t\"net/http\"\n\t\"os\"\n\n\t\"github.com/labstack/echo/v5\"\n)\n\nfunc main() {\n\tapp := echo.New()\n\tapp.GET(\"/\", func(c *echo.Context) error {\n\t\treturn c.JSON(http.StatusOK, map[string]string{\"name\": \"MagiCore Echo\", \"status\": \"ok\"})\n\t})\n\tapp.GET(\"/api/health\", func(c *echo.Context) error {\n\t\treturn c.JSON(http.StatusOK, map[string]string{\"status\": \"ok\"})\n\t})\n\tport := os.Getenv(\"PORT\")\n\tif port == \"\" {\n\t\tport = \"4315\"\n\t}\n\tif err := app.Start(\"0.0.0.0:\" + port); err != nil && err != http.ErrServerClosed {\n\t\tpanic(err)\n\t}\n}\n",
        ),
        "fiber" => (
            "github.com/gofiber/fiber/v3",
            "v3.6.1",
            "package main\n\nimport (\n\t\"os\"\n\n\t\"github.com/gofiber/fiber/v3\"\n)\n\nfunc main() {\n\tapp := fiber.New()\n\tapp.Get(\"/\", func(c fiber.Ctx) error {\n\t\treturn c.JSON(map[string]string{\"name\": \"MagiCore Fiber\", \"status\": \"ok\"})\n\t})\n\tapp.Get(\"/api/health\", func(c fiber.Ctx) error {\n\t\treturn c.JSON(map[string]string{\"status\": \"ok\"})\n\t})\n\tport := os.Getenv(\"PORT\")\n\tif port == \"\" {\n\t\tport = \"4315\"\n\t}\n\tif err := app.Listen(\"0.0.0.0:\" + port); err != nil {\n\t\tpanic(err)\n\t}\n}\n",
        ),
        "gin" => (
            "github.com/gin-gonic/gin",
            "v1.12.0",
            "package main\n\nimport (\n\t\"net/http\"\n\t\"os\"\n\n\t\"github.com/gin-gonic/gin\"\n)\n\nfunc main() {\n\tapp := gin.New()\n\tapp.Use(gin.Logger(), gin.Recovery())\n\tapp.GET(\"/\", func(c *gin.Context) {\n\t\tc.JSON(http.StatusOK, gin.H{\"name\": \"MagiCore Gin\", \"status\": \"ok\"})\n\t})\n\tapp.GET(\"/api/health\", func(c *gin.Context) {\n\t\tc.JSON(http.StatusOK, gin.H{\"status\": \"ok\"})\n\t})\n\tport := os.Getenv(\"PORT\")\n\tif port == \"\" {\n\t\tport = \"4315\"\n\t}\n\tif err := app.Run(\"0.0.0.0:\" + port); err != nil {\n\t\tpanic(err)\n\t}\n}\n",
        ),
        _ => bail!("unsupported native Go Web framework: {framework}"),
    };
    write_file(
        &target.join("go.mod"),
        &format!("module {module_name}\n\ngo 1.26.0\n\nrequire {module} {version}\n"),
    )?;
    write_file(&target.join("main.go"), source)
}
