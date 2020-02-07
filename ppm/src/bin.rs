extern crate libppm;

use std::fs::File;
use std::path::Path;
use libppm::Image;

pub fn main() {

    /* Read the image we'll use the library on*/
    let image_name: String = "image.ppm";

    println!("********************** Reading the image: {} **********************", image_name);
    let image: Image = Image::new_with_file(Path::new(image_name));
    println!("Input image successfully read.");

    /* Invert the image */
    println!("********************** Inverting the loaded image: **********************");
    let inverted_image: Image = image.invert_image();
    println!("Invert successfull.");

    /* Invert the image */
    println!("********************** Grayscaling the loaded image: **********************");
    let grayscaled_image: Image = image.grayscale_image();
    println!("Grayscaling successfull.");

    /* Save the resulting images */
    println!("********************** saving the resulting images: **********************");
    let inverted_image_name: String = "inverted_image.ppm";
    let grayscaled_image_name: String = "grayscaled_image.ppm";

    inverted_image.save(inverted_image_name);
    println!("Inverted image successfully saved into: {} .", inverted_image_name);

    grayscaled_image.save(grayscaled_image_name);
    println!("Grayscaled image successfully saved into: {} .", grayscaled_image_name);

    println!("Thank's !!");

}