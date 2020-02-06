#[derive(Clone, Copy)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            red: red,
            green: green,
            blue: blue,
        }
    }

    pub fn display(self) -> String {
        format!("(r:{}, g:{}, b:{})", self.red, self.green, self.blue)
    }

    pub fn invert(& mut self){
        self.red = self.red - 255;
        self.green = self.green - 255;
        self.blue = self.blue - 255;

    }

    fn eq(self, other: Pixel) -> bool {
         self.blue == other.blue && self.red == other.red && self.green == other.green
    }
    pub fn grayscale(&mut self) {
        self.red = self.red / 3;
        self.green = self.green / 3;
        self.blue = self.blue / 3;

    }
}

pub fn red(pix: Pixel)-> u8{
    pix.red
}
pub fn green(pix: Pixel)-> u8{
    pix.green
}
pub fn blue(pix: Pixel)-> u8{
    pix.blue
}
impl PartialEq for Pixel{
    fn eq(&self,other:&Pixel)->bool{
        return self.red==other.red || self.green==other.green || self.blue==other.blue;
    }
}

pub struct Image{
    vector:Vec<Pixel>,
    width:usize,
    height:usize
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

    #[test]
    fn test_red() {
        assert_eq!(red(get_sample_pixel()), 8)
    }

    #[test]
    fn test_green() {
        assert_eq!(green(get_sample_pixel()), 12)
    }

    #[test]
    fn test_blue() {
        assert_eq!(blue(get_sample_pixel()), 16)
    }

    #[test]
    fn test_display() {
        assert_eq!(get_sample_pixel().display(), "(r:8, g:12, b:16)")
    }
}
