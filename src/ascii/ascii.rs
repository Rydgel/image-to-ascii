extern crate image;

use cmd::Command;
use ascii::Result;
use image::GenericImage;
use ascii::AsciiError;

// 70 levels of gray
const GSCALE1: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^;`'. ";
// 10 levels of gray
const GSCALE2: &'static str =
    "@%#*+=-:. ";

#[derive(Debug, Eq, PartialEq)]
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
        let gscale1: Vec<char> = GSCALE1.chars().collect();
        let gscale2: Vec<char> = GSCALE2.chars().collect();
        println!("Image dimensions ({:?}, {:?})", width, height);
        // convert to grayscale
        let grey_img = img.grayscale();
        // compute tile width
        let w = width / self.cols();
        // compute tile height based on the aspect ratio and scale of the font
        let h = w as f32 / self.scale();
        // compute number of rows to use in the final grid
        let rows = (height as f32 / h) as u32;
        println!("cols: {:?}, rows: {:?}", self.cols(), rows);
        println!("tile dims: {:?} x {:?}", w, h);

        if *self.cols() > width || rows > height {
            // todo make a new error type for this
            // Image too small for specified cols!
            return Err(AsciiError::ImageError);
        }

        // an ASCII image is a list of character strings
        let mut aimg: Vec<char> = Vec::new();

        for j in 0..(rows - 1) {
            let y1 = j * h as u32;
            let mut y2 = ((j + 1) * h as u32) as u32;
            // correct the last tile
            if j == rows - 1 {
                y2 = height;
            }

            for i in 0..(*self.cols() - 1) {
                // crop the image to fit the tile
                let x1 = (i * w) as u32;
                let mut x2 = ((i+1) * w) as u32;
                // correct the last tile
                if i == *self.cols() - 1 {
                    x2 = width;
                }
                // crop the image to extract the tile into another Image object
                let crop_img = crop_image(&grey_img, x1, y1, x2, y2);
                // get the average luminance
                let average = get_average(&crop_img);
                // look up the ASCII character for grayscale value (avg)
                let gsval;
                if *self.ascii_type() == AsciiType::Complex {
                    gsval = gscale1.get(((average * 69)/255) as usize).unwrap();
                } else {
                    gsval = gscale2.get(((average * 9)/255) as usize).unwrap();
                }

                aimg.push(*gsval);
            }
        }

        let s: String = aimg.into_iter().collect();
        Ok(s.to_string())
    }
}

fn crop_image(img: &image::DynamicImage, x: u32, y: u32, width: u32, height: u32) -> image::DynamicImage {
    let mut img = img.clone();
    return img.crop(x, y, width, height);
}

fn get_average(img: &image::DynamicImage) -> i32 {
    let pixels = img.raw_pixels();
    let sum = pixels.iter()
        .map(|a| *a as f32)
        .fold(0.0, |a, b| a + b);
    let length = pixels.as_slice().len() as f32;
    return (sum / length).round() as i32;
}
