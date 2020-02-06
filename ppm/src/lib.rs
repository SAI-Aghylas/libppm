#[derive(Clone, Copy, PartialEq, Debug)]
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

    fn red(self) -> u8 {
        self.red
    }
    fn green(self) -> u8 {
        self.green
    }
    fn blue(self) -> u8 {
        self.blue
    }

    fn display(self) -> String {
        format!("(r:{}, g:{}, b:{})", self.red, self.green, self.blue)
    }

    fn invert(&self) -> Pixel {
        Pixel::new(255 - self.red(), 255 - self.green(), 255 - self.blue())
    }

    fn eq(self, other: Pixel) -> bool {
        self.blue == other.blue && self.red == other.red && self.green == other.green
    }

    fn partial_eq(self, other: Pixel) -> bool {
        self.blue == other.blue || self.red == other.red || self.green == other.green
    }

    fn grayscale(&self) -> Pixel {
        Pixel::new(self.red() / 3, self.green() / 3, self.blue() / 3)
    }
}

#[derive(Clone)]
pub struct Image {
    vector: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(vector: Vec<Pixel>, width: usize, height: usize) -> Image {
        Image {
            vector: vector,
            width: width,
            height: height,
        }
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
        Image::new(vec![get_sample_pixel()], 12, 16)
    }

    #[test]
    fn test_red() {
        assert_eq!(get_sample_pixel().red(), 8)
    }

    #[test]
    fn test_green() {
        assert_eq!(get_sample_pixel().green(), 12)
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
    fn test_vecto() {
        assert_eq!(get_sample_image().vector(), vec![get_sample_pixel()])
    }
    #[test]
    fn test_width() {
        assert_eq!(get_sample_image().width, 12)
    }
    #[test]
    fn test_height() {
        assert_eq!(get_sample_image().height, 16)
    }

    #[test]
    fn test_revert() {
        let sample_pix: Pixel = get_sample_pixel();
        let inverted_pix: Pixel = sample_pix.invert();
        assert_eq!(inverted_pix.red(), 255 - sample_pix.red());
        assert_eq!(inverted_pix.green(), 255 - sample_pix.green());
        assert_eq!(inverted_pix.blue(), 255 - sample_pix.blue());
    }

    #[test]
    fn test_grayscale() {
        let sample_pix: Pixel = get_sample_pixel();
        let grayscaled_pix: Pixel = sample_pix.grayscale();
        assert_eq!(grayscaled_pix.red(), sample_pix.red() / 3);
        assert_eq!(grayscaled_pix.green(), sample_pix.green() / 3);
        assert_eq!(grayscaled_pix.blue(), sample_pix.blue() / 3);
    }

    #[test]
    fn test_eq_pixel() {
        let pix_1: Pixel = get_sample_pixel();
        let pix_2: Pixel = get_sample_pixel();
        let pix_3: Pixel = get_sample_pixel().invert();

        assert_eq!(pix_1.eq(pix_2), true);
        assert_eq!(pix_1.eq(pix_3), false);
    }

    #[test]
    fn test_partial_eq_pixel() {
        let pix_1: Pixel = get_sample_pixel();
        let pix_2: Pixel = get_sample_pixel();
        let pix_3: Pixel = get_sample_pixel().invert();

        assert_eq!(pix_1.partial_eq(pix_2), true);
        assert_eq!(pix_1.partial_eq(pix_3), false);
    }
}
