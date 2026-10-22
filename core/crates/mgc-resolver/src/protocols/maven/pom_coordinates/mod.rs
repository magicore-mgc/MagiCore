//! Parent/project coordinate parsing and property substitution.
//! Đọc tọa độ parent/project và thay property.

mod pom_coordinates;
pub use pom_coordinates::{parse_parent_coords, pom_project_coordinates, substitute_properties};
