mod ascii;
mod output;
mod error;

pub use ascii::error::AsciiError;
pub use ascii::ascii::AsciiType;
pub use ascii::ascii::Ascii;

pub type AsciiString = String;
pub type Result = ::std::result::Result<AsciiString, AsciiError>;
