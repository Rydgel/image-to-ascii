use ascii::AsciiString;
use std::fs::File;
use std::io::Write;
use ascii::error::AsciiError;


pub trait Output {
    fn save_on_disk(&self, output: String) -> ::std::result::Result<String, AsciiError>;
    fn show_stdout(&self);
}

impl Output for AsciiString {
    fn save_on_disk(&self, output: String) -> ::std::result::Result<String, AsciiError> {
        let mut f = try!(File::create(output));
        try!(f.write_all(self.as_bytes()));
        return Ok("Output saved.".to_string());
    }

    fn show_stdout(&self) {
        print!("{:?}", self);
    }
}
