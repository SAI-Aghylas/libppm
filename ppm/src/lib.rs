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
}


pub fn red(pix: Pixel)-> u8{
    pix.red
}
pub fn green(pix: Pixel)-> u8{
    pix.red
}
pub fn blue(pix: Pixel)-> u8{
    pix.red
}

use std::fmt;

impl fmt::Display for Pixel{
    fn fmt(&self, f:&mut fmt:: Formatter<'_>) -> fmt::Result {
        write!(f,"(r:{}, g:{}, b:{})", self.red, self.green, self.blue)

    }
}