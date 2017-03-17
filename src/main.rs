//! An example of opening an image.
extern crate image;
extern crate rand;

use std::env;
//use std::fs::File;
use std::path::Path;
use std::cmp;
use std::u8;

use image::GenericImage;



fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        "D:\\Pictures\\wp\\1447560296523.jpg".to_string()
    };

    // Use the open function to load an image from a PAth.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new(&file)).unwrap();

    // The dimensions method returns the images width and height
    let (width, height) = im.dimensions();
    println!("dimensions {}x{}", width,height);


    // The color method returns the image's ColorType
    println!("{:?}", im.color());
    let mut count_black = 0;
    let mut count_white = 0;
    let mut count_gray = 0;
    let mut count_r = 0;
    let mut count_g = 0;
    let mut count_b = 0;
    let mut count_y = 0;
    let mut count_m = 0;
    let mut count_c = 0;

    let thresh = 10;

    for ( _, _, pixel) in im.pixels() {
        let (r, g, b) = (pixel.data[0], pixel.data[1], pixel.data[2]);

        if r <= thresh && g <= thresh && b <= thresh {
            count_black += 1;
        }
        else if r >= (u8::MAX - thresh) && g >= (u8::MAX - thresh) && b >= (u8::MAX - thresh) {
            count_white += 1;
        }
        else {
            let max_pix = cmp::max(r,cmp::max(g,b));
            if r >= (max_pix - thresh) && g >= (max_pix - thresh) && b >= (max_pix - thresh) {
                count_gray += 1;
            }
            else if r >= (max_pix - thresh) && g >= (max_pix - thresh) {
                count_y += 1;
            }
            else if r >= (max_pix - thresh) && b >= (max_pix - thresh) {
                count_m += 1;
            }
            else if b >= (max_pix - thresh) && g >= (max_pix - thresh) {
                count_c += 1;
            }
            else if r == max_pix {
                count_r += 1;
            }
            else if g == max_pix {
                count_g += 1;
            }
            else if b == max_pix {
                count_b += 1;
            }
        }
    }
    println!("counted \
             {} black pixels, {} gray pixels, {} white pixels, \
             {} yellow pixels, {} magenta pixels, {} cyan pixels, \
             {} red pixels, {} green pixels, {} blue pixels", 
        count_black, count_gray, count_white, 
        count_y, count_m, count_c,
        count_r, count_g, count_b);
//    let ref mut fout = File::create(&Path::new("D:\\Documents\\GitHub\\image-test\\res\\abc.jpg")).unwrap();

    // Write the contents of this image to the Writer in PNG format.
//    let _ = im.save(fout, image::JPEG).unwrap();
}
