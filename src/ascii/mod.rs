mod ascii;
mod error;
mod output;

pub use ascii::error::AsciiError;
pub use ascii::ascii::AsciiType;
pub use ascii::ascii::Ascii;
pub use ascii::output::Output;

pub type AsciiString = String;
pub type Result = ::std::result::Result<AsciiString, AsciiError>;
