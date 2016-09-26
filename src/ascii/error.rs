use std::fmt;

#[derive(Debug)]
pub enum AsciiError {
    IoError(::std::io::Error),
    ImageError,
}

impl From<::std::io::Error> for AsciiError {
    fn from(error: ::std::io::Error) -> AsciiError {
        AsciiError::IoError(error)
    }
}

impl From<::image::ImageError> for AsciiError {
    fn from(_: ::image::ImageError) -> AsciiError {
        AsciiError::ImageError
    }
}

impl fmt::Display for AsciiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AsciiError::IoError(ref e) => write!(f, "{}", e),
            AsciiError::ImageError => write!(f, "The image supplied is invalid."),
        }
    }
}

impl ::std::error::Error for AsciiError {
    fn description(&self) -> &str {
        match *self {
            AsciiError::IoError(_) => "An IO error occured when attempted to convert the image",
            AsciiError::ImageError => "The image supplied is invalid.",
        }
    }
}
