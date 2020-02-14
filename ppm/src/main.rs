extern crate libppm;

use libppm::Image;
use std::path::Path;

pub fn main() {
    /* Read the image we'll use the library on*/
    let image_name: &str = "..\\superman.ppm";

    println!(
        "********************** Reading the image: {} **********************",
        image_name
    );
    let image: Image = Image::new_with_file(Path::new(image_name));
    println!("Input image successfully read.");
    /* Invert the image */
    println!("********************** Inverting the loaded image: **********************");
    let mut inverted_image: Image = image.clone();
    inverted_image.invert_image();
    println!("Invert successfull.");

    /* Invert the image */
    println!("********************** Grayscaling the loaded image: **********************");
    let mut grayscaled_image: Image = image.clone();
    grayscaled_image.grayscale_image();
    println!("Grayscaling successfull.");

    /* Save the resulting images */
    println!("********************** saving the resulting images: **********************");
    let inverted_image_name: &str = "..\\inverted_superman.ppm";
    let grayscaled_image_name: &str = "..\\grayscaled_superman.ppm";

    inverted_image.save(Path::new(inverted_image_name));
    println!(
        "Inverted image successfully saved into: {} .",
        inverted_image_name
    );

    grayscaled_image.save(Path::new(grayscaled_image_name));
    println!(
        "Grayscaled image successfully saved into: {} .",
        grayscaled_image_name
    );

    println!("Thank's !!");
}
