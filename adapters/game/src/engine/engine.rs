//! Game engine identities and their accepted string forms.
//! Định danh game engine và các chuỗi được chấp nhận.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEngine {
    Bevy,
    Godot,
    Unity,
    Unreal,
}

impl GameEngine {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "bevy" => Some(Self::Bevy),
            "godot" => Some(Self::Godot),
            "unity" => Some(Self::Unity),
            "unreal" => Some(Self::Unreal),
            _ => None,
        }
    }

    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Bevy => "bevy",
            Self::Godot => "godot",
            Self::Unity => "unity",
            Self::Unreal => "unreal",
        }
    }
}

#[cfg(test)]
#[path = "engine/test/mod.rs"]
mod tests;
