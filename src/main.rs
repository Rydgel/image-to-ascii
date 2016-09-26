extern crate image;
#[macro_use] extern crate clap;

mod ascii;
mod cmd;

use ascii::Ascii;

fn main() {
    let command = cmd::read_command();
    println!("{:?}", command);
    println!("{:?}", command.convert());
}
