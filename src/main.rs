// mod args;
mod chunk;
mod chunk_type;
mod png_util;
// mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub fn throw_string_error(s: &'static str) -> Error {
    return std::io::Error::new(std::io::ErrorKind::Other, "Bad Header").into();
}
fn main() -> Result<()> {
    todo!()
}
