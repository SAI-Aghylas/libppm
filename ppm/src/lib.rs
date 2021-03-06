#![feature(test)]
extern crate test;
use test::Bencher;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
/// La structure Pixel représente l'encodage d'un pixel sur 24 bits (8 bits par élément du tuple rgb)
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    /// Crée un nouveau Pixel à partir des éléments passés en argument
    /// # Arguments
    /// * red: u8
    /// * green: u8
    /// * blue: u8
    /// # Return
    /// * Pixel
    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {
        Pixel { red, green, blue }
    }

    /// Cette fonction retourne la valeur du champ `red` d'un Pixel
    /// # Return
    /// * `Pixel.red`: u8
    pub fn red(self) -> u8 {
        self.red
    }

    /// Cette fonction retourne la valeur du champ `green` d'un Pixel
    /// # Return
    /// * `Pixel.green`: u8
    pub fn green(self) -> u8 {
        self.green
    }

    /// Cette fonction retourne la valeur du champ `blue` d'un Pixel
    /// # Return
    /// * `Pixel.blue`: u8
    pub fn blue(self) -> u8 {
        self.blue
    }

    /// `display` retourne un affichage d'un Pixel
    /// # Return
    /// description : String
    pub fn display(self) -> String {
        format!("(r:{}, g:{}, b:{})", self.red, self.green, self.blue)
    }

    /// Compare deux Pixels
    /// # Return
    /// `true` si les deux pixels sont pareils.
    pub fn eq(self, other: Pixel) -> bool {
        self.blue == other.blue && self.red == other.red && self.green == other.green
    }

    /// Compare partiellement  deux Pixels
    /// # Return
    /// `true` si les deux pixels sont partiellement pareils.
    pub fn partial_eq(self, other: Pixel) -> bool {
        self.blue == other.blue || self.red == other.red || self.green == other.green
    }

    /// `invert_pixel` inverse les valeurs d'un Pixel, dans le but d'avoir le négatif du pixel.
    pub fn invert_pixel(&mut self) {
        self.red = 255 - self.red();
        self.green = 255 - self.green();
        self.blue = 255 - self.blue();
    }

    /// `grayscale_pixel` transforme un Pixel en noir et blanc au lieu de RGB.
    pub fn grayscale_pixel(&mut self) {
        let mean: u8 = (self.red() / 3) + (self.green() / 3) + (self.blue() / 3);

        self.red = mean;
        self.green = mean;
        self.blue = mean;
    }
}

impl FromStr for Pixel {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(' ').collect();

        let red_str = coords[0].parse::<u8>()?;
        let green_str = coords[1].parse::<u8>()?;
        let blue_str = coords[2].parse::<u8>()?;

        Ok(Pixel {
            red: red_str,
            green: green_str,
            blue: blue_str,
        })
    }
}
#[derive(Clone)]
/// La structure `Image` représente une image
/// # Fields
/// * `vector`: Vecteur des pixels de l'image
/// * `width`: largeur de l'image, en nombre de pixels
/// * `height`: hauteur de l'image, en nombre de pixels
pub struct Image {
    vector: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Image {
    /// Cette fonction crée une nouvelle Image à partir des éléments passés en argument
    /// # Arguments
    /// * `vector`: Vecteur des pixels de l'image
    /// * `width`: largeur de l'image, en nombre de pixels
    /// * `height`: hauteur de l'image, en nombre de pixels
    /// # Return
    /// Une nouvelle Image
    pub fn new(vector: Vec<Pixel>, width: usize, height: usize) -> Image {
        Image {
            vector,
            width,
            height,
        }
    }

    /// Compare deux Images
    /// # Return
    /// `true` si les deux Images sont pareils.
    pub fn eq(self, other: Image) -> bool {
        let mut b = true;
        if self.width == other.width
            && self.height == other.height
            && self.vector.len() == other.vector.len()
        {
            let mut i = 0;
            loop {
                if i >= self.vector.len() {
                    break;
                };
                if !self.vector[i].eq(other.vector[i]) {
                    break b = false;
                }
                i += 1;
            }
        } else {
            b = false;
        };
        b
    }

    /// `invert_image` inverse les pixels d'une image, dans le but d'avoir le négatif de l'image.
    pub fn invert_image(&mut self) {
        let mut i = 0;
        loop {
            if i >= self.vector.len() {
                break;
            };
            self.vector[i].invert_pixel();
            i += 1;
        }
    }

    /// `grayscale_image` transforme l'image du RGB en noir et blanc.
    pub fn grayscale_image(&mut self) {
        let mut i = 0;
        loop {
            if i >= self.vector.len() {
                break;
            };
            self.vector[i].grayscale_pixel();
            i += 1;
        }
    }

    /// Cette fonction retourne le vecteur de pixels d'une image
    /// # Return
    /// * `Image.vector`: vec<Pixel>
    pub fn vector(self) -> Vec<Pixel> {
        self.vector
    }

    /// Cette fonction retourne la largeur d'une image
    /// # Return
    /// * `Image.width`: usize
    pub fn width(self) -> usize {
        self.width
    }

    /// Cette fonction retourne la hauteur d'une image
    /// # Return
    /// * `Image.height`: usize
    pub fn height(self) -> usize {
        self.height
    }

    /// `save` enregistre une Image dans le fichier spécifié
    /// # Argument
    /// * file_name: Fichier de sortie
    /// # Return
    /// * Nombre de Pixels enregistrés
    pub fn save(self, file_name: &Path) -> i32 {
        let mut file: File = match File::create(file_name) {
            Ok(file) => file,
            Err(err) => panic!("error when saving image: {}", err),
        };

        let mut nb_written: i16 = 0;
        let mut nb_saved_pixels: i32 = 0;

        let img_vec: Vec<Pixel> = self.clone().vector();
        let img_heigt: usize = self.clone().height();
        let img_width: usize = self.clone().width();

        write!(file, "P3\n{} {}\n{}\n", img_width, img_heigt, 255);

        let mut nb_pixels_in_cur_line = 0;
        for val in img_vec.iter() {
            write!(file, "{} {} {}", val.red(), val.green(), val.blue());
            nb_pixels_in_cur_line += 1;
            nb_written += 1;
            nb_saved_pixels += 1;

            if nb_written % (img_width as i16) == 0 {
                write!(file, "\n");
                nb_written = 0;
                nb_pixels_in_cur_line = 0;
            } else if (nb_pixels_in_cur_line == 5){
                nb_pixels_in_cur_line = 0;
                write!(file,"\n");
            }else {
                write!(file, "\t");
            }
        }

        nb_saved_pixels
    }

    /// `new_with_file` Charge une image à partir d'un fichier
    /// # Argument
    /// * file_name: Image à charger
    /// # Return
    /// * Image contenues dans le fichier
    pub fn new_with_file(file_name: &Path) -> Image {
        let file: File = match File::open(file_name) {
            Ok(file) => file,
            Err(err) => panic!("error when opening image: {}", err),
        };

        let lines = BufReader::new(file).lines();

        let mut image: Image = Image::new(vec![], 0, 0);

        let mut vector: Vec<Pixel> = Vec::new();

        let mut read_lines = 0;
        for line in lines {
            read_lines += 1;
            match line {
                Ok(l) => {
                    if read_lines == 1 || read_lines == 3 {
                        println!(" *** skipping unneeded file line *** ")
                    } else if read_lines == 2 {
                        let mut cleaned_line: String = l.replace("\n", "");
                        let height_and_widht: Vec<&str> = cleaned_line.split(" ").collect();
                        image.width = height_and_widht[0].parse::<usize>().unwrap();
                        image.height = height_and_widht[1].parse::<usize>().unwrap();
                    } else {
                        let mut cleaned_line: String = l.replace("\n", "");
                        let pixels_str: Vec<&str> = cleaned_line.split("\t").collect();
                        for pixel in pixels_str {
                            vector.push(Pixel::from_str(pixel).unwrap());
                        }
                    }
                }
                Err(err) => panic!("error when unwrapping an image line: {}", err),
            }
        }
        image.vector = vector;

        image
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_sample_pixel() -> Pixel {
        Pixel::new(8, 12, 16)
    }

    fn get_sample_image() -> Image {
        Image::new(
            vec![
                get_sample_pixel(),
                get_sample_pixel(),
                get_sample_pixel(),
                get_sample_pixel(),
                get_sample_pixel(),
                get_sample_pixel(),
            ],
            3,
            2,
        )
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
        assert_eq!(
            get_sample_image().vector(),
            vec![
                get_sample_pixel(),
                get_sample_pixel(),
                get_sample_pixel(),
                get_sample_pixel(),
                get_sample_pixel(),
                get_sample_pixel()
            ]
        )
    }
    #[test]
    fn test_width() {
        assert_eq!(get_sample_image().width, 3)
    }
    #[test]
    fn test_height() {
        assert_eq!(get_sample_image().height, 2)
    }

    #[test]
    fn test_invert_pixel() {
        let sample_pix: Pixel = get_sample_pixel();
        let mut inverted_pix: Pixel = sample_pix.clone();
        inverted_pix.invert_pixel();
        assert_eq!(inverted_pix.red(), 255 - sample_pix.red());
        assert_eq!(inverted_pix.green(), 255 - sample_pix.green());
        assert_eq!(inverted_pix.blue(), 255 - sample_pix.blue());
    }

    #[test]
    fn test_grayscale_pixel() {
        let sample_pix: Pixel = get_sample_pixel();
        let mut grayscaled_pix: Pixel = sample_pix.clone();
        grayscaled_pix.grayscale_pixel();
        assert_eq!(
            grayscaled_pix.red(),
            (sample_pix.red() / 3) + (sample_pix.green() / 3) + (sample_pix.blue() / 3)
        );
        assert_eq!(
            grayscaled_pix.green(),
            (sample_pix.red() / 3) + (sample_pix.green() / 3) + (sample_pix.blue() / 3)
        );
        assert_eq!(
            grayscaled_pix.blue(),
            (sample_pix.red() / 3) + (sample_pix.green() / 3) + (sample_pix.blue() / 3)
        );
    }

    #[test]
    fn test_eq_pixel() {
        let pix_1: Pixel = get_sample_pixel();
        let pix_2: Pixel = get_sample_pixel();
        let mut pix_3: Pixel = get_sample_pixel().clone();
        pix_3.invert_pixel();

        assert_eq!(pix_1.eq(pix_2), true);
        assert_eq!(pix_1.eq(pix_3), false);
    }

    #[test]
    fn test_partial_eq_pixel() {
        let pix_1: Pixel = get_sample_pixel();
        let pix_2: Pixel = get_sample_pixel();
        let mut pix_3: Pixel = get_sample_pixel().clone();
        pix_3.invert_pixel();

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
        let mut inverted_img: Image = sample_img.clone();
        inverted_img.invert_image();
        let mut j = 0;
        loop {
            if j == sample_img.vector.len() {
                break;
            };
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
        }
    }

    #[test]
    fn test_grayscale_image() {
        let sample_img: Image = get_sample_image();
        let mut graycaled_img: Image = sample_img.clone();
        graycaled_img.grayscale_image();
        let mut j = 0;
        let mut graycaled_val = 0;
        loop {
            if j >= sample_img.vector.len() {
                break;
            };
            graycaled_val = sample_img.vector[j].red() / 3
                + sample_img.vector[j].green() / 3
                + sample_img.vector[j].blue() / 3;
            assert_eq!(graycaled_img.vector[j].red(), graycaled_val);
            assert_eq!(graycaled_img.vector[j].green(), graycaled_val);
            assert_eq!(graycaled_img.vector[j].blue(), graycaled_val);
            j += 1;
        }
    }
    #[test]
    fn test_save_image() {
        assert_eq!(
            get_sample_image().save(Path::new("image_from_test.ppm")),
            (get_sample_image().width() * get_sample_image().height()) as i32
        )
    }

    #[test]
    fn test_new_with_file() {
        let img = get_sample_image();
        let read_img = Image::new_with_file(Path::new("image_from_test.ppm"));

        assert_eq!(read_img.eq(img), true);
    }

    //Begin Benchmarks:
    #[bench]
    fn bench_test_red(b: &mut Bencher) {
        b.iter(|| get_sample_pixel().red());
    }

    #[bench]
    fn bench_test_green(b: &mut Bencher) {
        b.iter(|| get_sample_pixel().green());
    }

    #[bench]
    fn bench_test_blue(b: &mut Bencher) {
        b.iter(|| get_sample_pixel().blue());
    }

    #[bench]
    fn bench_test_display(b: &mut Bencher) {
        b.iter(|| get_sample_pixel().display());
    }

    #[bench]
    fn bench_test_vector(b: &mut Bencher) {
        b.iter(|| get_sample_image().vector());
    }

    #[bench]
    fn bench_test_width(b: &mut Bencher) {
        b.iter(|| get_sample_image().width());
    }

    #[bench]
    fn bench_test_height(b: &mut Bencher) {
        b.iter(|| get_sample_image().height());
    }

    #[bench]
    fn bench_test_invert_pixel(b: &mut Bencher) {
        b.iter(|| get_sample_pixel().invert_pixel());
    }

    #[bench]
    fn bench_test_grayscale_pixel(b: &mut Bencher) {
        b.iter(|| get_sample_pixel().grayscale_pixel());
    }
    #[bench]
    fn bench_test_eq_pixel(b: &mut Bencher) {
        b.iter(|| get_sample_pixel().eq(get_sample_pixel()));
    }
    #[bench]
    fn bench_test_partial_eq_pixel(b: &mut Bencher) {
        b.iter(|| get_sample_pixel().partial_eq(get_sample_pixel()));
    }
    #[bench]
    fn bench_test_eq_image(b: &mut Bencher) {
        b.iter(|| get_sample_image().eq(get_sample_image()));
    }
    #[bench]
    fn bench_test_invert_image(b: &mut Bencher) {
        b.iter(|| get_sample_image().invert_image());
    }
    #[bench]
    fn bench_test_grayscale_image(b: &mut Bencher) {
        b.iter(|| get_sample_image().grayscale_image());
    }
    #[bench]
    fn bench_test_save_image(b: &mut Bencher) {
        b.iter(|| get_sample_image().save(Path::new("image_from_test.ppm")));
    }
    #[bench]
    fn bench_test_new_with_file(b: &mut Bencher) {
        let img = Image::new_with_file(Path::new("image_from_test.ppm"));
        b.iter(|| &img);
    }
}
