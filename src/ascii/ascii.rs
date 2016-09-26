extern crate image;

use std::io;
use std::path::Path;
use cmd::Command;
use ascii::Result;
use image::GenericImage;

// 70 levels of gray
const GSCALE1: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^;`'. ";
// 10 levels of gray
const GSCALE2: &'static str =
    "@%#*+=-:. ";

#[derive(Debug)]
pub enum AsciiType {
    Simple,
    Complex,
}

pub trait Ascii {
    fn convert(&self) -> Result;
}

impl Ascii for Command {
    fn convert(&self) -> Result {
        let img = try!(image::open(self.source()));
        // store the image dimensions
        let (width, height) = img.dimensions();
        // convert to grayscale
        let grey_img = img.grayscale();
        
        let average = get_average(img);
        Ok("lol".to_string())
    }
}

fn get_average(img: image::DynamicImage) -> i32 {
    let pixels = img.raw_pixels();
    let sum = pixels.iter()
        .map(|a| *a as f32)
        .fold(0.0, |a, b| a + b);
    let length = pixels.as_slice().len() as f32;
    return (sum / length).round() as i32;
}
