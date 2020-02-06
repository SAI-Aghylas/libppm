#[derive(Clone, Copy,Debug)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            red: red,
            green: green,
            blue: blue,
        }
    }

    fn display(self) -> String {
        format!("(r:{}, g:{}, b:{})", self.red, self.green, self.blue)
    }

    fn eq(self, other: Pixel) -> bool {
        self.blue == other.blue && self.red == other.red && self.green == other.green
    }
    fn grayscale(&mut self) {
        self.red = self.red / 3;
        self.green = self.green / 3;
        self.blue = self.blue / 3;
    }
    fn red(pix: Pixel) -> u8 {
        pix.red
    }
    fn green(pix: Pixel) -> u8 {
        pix.green
    }
    fn blue(pix: Pixel) -> u8 {
        pix.blue
    }

}

impl PartialEq for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        return (self.red == other.red || self.green == other.green || self.blue == other.blue);
    }
}

#[derive(Debug,Clone)]
pub struct Image {
    vector: Vec<Pixel>,
    width: usize,
    height: usize,
}

use std::fs::File;
use std::io::prelude::*;

impl Image {
    fn new( v_pix: Vec<Pixel>,width: usize, height: usize) -> Image {
        Image {
            vector: v_pix,
            width: width,
            height: height,
        }
    }

    fn eq(self, other: Image) -> bool {

        self.vector.len() == other.vector.len() && self.width == other.width && self.height == other.height
    }

    fn vector(self) -> Vec<Pixel> {
        self.vector
    }
    fn width(self) -> usize {
        self.width
    }
    fn height(self) -> usize {
        self.height
    }
    fn new_with_file(filename: &Path) -> Image{
        let mut file = File::open(&Path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        contents[];
        println!(contents);
        let mut image:Image::new();
        return image;


    }
}

//use std::fmt;
//
////impl fmt::Display for Pixel{
////    fn fmt(&self, f:&mut fmt:: Formatter<'_>) -> fmt::Result {
////        write!(f,"(r:{}, g:{}, b:{})", self.red, self.green, self.blue)
////
////    }
////}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fmt::Display;

    fn get_sample_pixel() -> Pixel {
        Pixel::new(8, 12, 16)
    }
    fn get_sample_image() -> Image {
        Image::new(vec![get_sample_pixel()],2, 2, )
    }

    #[test]
    fn test_red() {
        assert_eq!(get_sample_pixel().red(), 8)
    }

    #[test]
    fn test_green() {
        assert_eq!((get_sample_pixel().green()), 12)
    }

    #[test]
    fn test_blue() {
        assert_eq!(get_sample_pixel().blue(), 16)
    }

    #[test]
    fn test_display() {
        assert_eq!(get_sample_pixel().display(), "(r:8, g:12, b:16)")
    }



    #[test]
    fn test_width(){
        assert_eq!(get_sample_image().width(),2)
    }
    fn test_height(){
        assert_eq!(get_sample_image().height(),2)
    }

    fn test_vector(){
        assert_eq!(get_sample_image().vector(),vec![])
    }
    #[test]
    fn test_read_from_file(){
        new_with_file("F:\\Rust-Project\\libppm\\ppm\\sampleppm.ppm")
    }
}
