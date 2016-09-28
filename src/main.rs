extern crate image;
#[macro_use] extern crate clap;

mod ascii;
mod cmd;

use ascii::Ascii;
use ascii::Output;

fn main() {
    let command = cmd::read_command();
    match command.convert() {
        Err(e) => println!("{:?}", e),
        Ok(ascii_string) => match ascii_string.save_on_disk(command.output()) {
            Ok(_) => println!("Done."),
            Err(e) => println!("{:?}", e),
        },
    }
}
