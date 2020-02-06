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

    fn invert_pixel(self) -> Pixel {
        Pixel::new(255 - self.red(), 255 - self.green(), 255 - self.blue())
    }

    fn eq(self, other: Pixel) -> bool {
        self.blue == other.blue && self.red == other.red && self.green == other.green
    }

    fn partial_eq(self, other: Pixel) -> bool {
        self.blue == other.blue || self.red == other.red || self.green == other.green
    }

    fn grayscale_pixel(self) -> Pixel {
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

    fn eq(self, other: Image) -> bool {
        let mut b = true;
        if (self.width == other.width
            && self.height == other.height
            && self.vector.len() == other.vector.len())
        {
            let mut i = 0;
            loop {
                if (i >= self.vector.len()) {
                    break;
                };
                if (!self.vector[i].eq(other.vector[i])) {
                    break b = false;
                }
                i += 1;
            }
        } else {
            b = false;
        };
        b
    }
    fn invert_image(self) -> Image {
        let mut vect_inv = vec![];
        let mut j = 0;
        for i in self.vector.iter() {
            vect_inv.push(i.invert_pixel());
            j += 1;
        }
        Image::new(vect_inv, self.width, self.height)
    }

    fn grayscale_image(self) -> Image {
        let mut vect_inv = vec![];
        let mut j = 0;
        for i in self.vector.iter() {
            vect_inv.push(i.grayscale_pixel());
            j += 1;
        }
        Image::new(vect_inv, self.width, self.height)
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
    fn test_vector() {
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
    fn test_invert_pixel() {
        let sample_pix: Pixel = get_sample_pixel();
        let inverted_pix: Pixel = sample_pix.invert_pixel();
        assert_eq!(inverted_pix.red(), 255 - sample_pix.red());
        assert_eq!(inverted_pix.green(), 255 - sample_pix.green());
        assert_eq!(inverted_pix.blue(), 255 - sample_pix.blue());
    }

    #[test]
    fn test_grayscale_pixel() {
        let sample_pix: Pixel = get_sample_pixel();
        let grayscaled_pix: Pixel = sample_pix.grayscale_pixel();
        assert_eq!(grayscaled_pix.red(), sample_pix.red() / 3);
        assert_eq!(grayscaled_pix.green(), sample_pix.green() / 3);
        assert_eq!(grayscaled_pix.blue(), sample_pix.blue() / 3);
    }

    #[test]
    fn test_eq_pixel() {
        let pix_1: Pixel = get_sample_pixel();
        let pix_2: Pixel = get_sample_pixel();
        let pix_3: Pixel = get_sample_pixel().invert_pixel();

        assert_eq!(pix_1.eq(pix_2), true);
        assert_eq!(pix_1.eq(pix_3), false);
    }

    #[test]
    fn test_partial_eq_pixel() {
        let pix_1: Pixel = get_sample_pixel();
        let pix_2: Pixel = get_sample_pixel();
        let pix_3: Pixel = get_sample_pixel().invert_pixel();

        assert_eq!(pix_1.partial_eq(pix_2), true);
        assert_eq!(pix_1.partial_eq(pix_3), false);
    }
    #[test]
    fn test_eq_image() {
        let img1: Image = get_sample_image();
        let img2: Image = Image::new(vec![get_sample_pixel()], 3, 4);
        let img3: Image = get_sample_image();
        let img4: Image = Image::new(vec![Pixel::new(5, 12, 16)], 12, 16);
        assert_eq!(img1.clone().eq(img3), true);
        assert_eq!(img1.clone().eq(img2), false);
        assert_eq!(img1.clone().eq(img4), false);
    }
    #[test]
    fn test_invert_image() {
        let sample_img: Image = get_sample_image();
        let inverted_img: Image = sample_img.clone().invert_image();
        let mut j = 0;
        loop {
            assert_eq!(
                inverted_img.vector[j].red(),
                255 - sample_img.vector[j].red()
            );
            assert_eq!(
                inverted_img.vector[j].green(),
                255 - sample_img.vector[j].green()
            );
            assert_eq!(
                inverted_img.vector[j].blue(),
                255 - sample_img.vector[j].blue()
            );
            j += 1;
            if (j == sample_img.vector.len()) {
                break;
            }
        }
    }

    #[test]
    fn test_grayscale_image() {
        let sample_img: Image = get_sample_image();
        let inverted_img: Image = sample_img.clone().grayscale_image();
        let mut j = 0;
        loop {
            assert_eq!(
                inverted_img.vector[j].red(),
                (sample_img.vector[j].red()) / 3
            );
            assert_eq!(
                inverted_img.vector[j].green(),
                (sample_img.vector[j].green()) / 3
            );
            assert_eq!(
                inverted_img.vector[j].blue(),
                (sample_img.vector[j].blue()) / 3
            );
            j += 1;
            if (j == sample_img.vector.len()) {
                break;
            }
        }
    }
}
