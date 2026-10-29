//! Keg prefix link owner wiring.
//! Wiring owner link prefix keg.

mod prefix;

pub use prefix::{
    PrefixLink, create_link_to, create_symlink, ensure_link_parents, link_target, place_link,
    plan_links, relative_target, remove_link,
};
