use std::{ marker, slice::Iter };

/// Iterate over an Enum's values in the order in which they
/// are defined (top down)
pub trait EnumIter {
    fn iterator() -> Iter<'static, Self> where Self: marker::Sized;
}

/// Generate an enum from a &str. If the &str matches an enum value,
/// return Some(value), otherwise, return None. (At some point,
/// this trait may go away in favor of the nightly-only TryFrom trait )
pub trait EnumFromStr: EnumIter {
    fn from_str(key: &str) -> Option<Self> where Self: marker::Sized;
}
