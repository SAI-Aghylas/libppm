//Manipulating IO, Memory, Concurrent programming with threads and exposing Rust function through Foreign Function Interface.
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
//deriving
#[derive(Clone, Copy)]
//struct
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
//Constructor
impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel {
            red: red,
            green: green,
            blue: blue,
        }
    }
}
//access red
impl Pixel {
    pub fn red(pix: Pixel) -> u8 {
        return pix.red;
    }
}
//access green
impl Pixel {
    pub fn green(pix: Pixel) -> u8 {
        return pix.green;
    }
}
//access blue
impl Pixel {
    pub fn blue(pix: Pixel) -> u8 {
        return pix.blue;
    }
}
//Implement Display
use std::fmt;
impl fmt::Display for Pixel {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "(R:{}, G:{}, B:{})", self.red, self.green, self.blue)
    }
}
//invert
pub fn invert(mut pixel: Pixel) -> Pixel {
    pixel.red = pixel.red - 255;
    pixel.green = pixel.green - 255;
    pixel.blue = pixel.blue - 255;

    return pixel;
}
impl Pixel {
    fn eq(self, other: Pixel) -> bool {
        return self.blue == other.blue && self.red == other.red && self.green == other.green;
    }
}
//Convert RGB to Grayscale
pub fn grayscale(mut pix: Pixel) -> Pixel {
    pix.red = pix.red / 3;
    pix.green = pix.green / 3;
    pix.blue = pix.blue / 3;
    return pix;
}
impl PartialEq for Pixel{
    fn eq(self,other:Pixel)->bool{
        return self.red==other.red
    }
}
