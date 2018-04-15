use std::{ marker, slice::Iter };

/// Provide a way to iterate over enum values and convert them from
/// str
pub trait EnumEase {
    fn iterator() -> Iter<'static, Self> where Self: marker::Sized;
    fn from_str(key: &str) -> Option<Self> where Self: marker::Sized;
}

