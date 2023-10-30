use std::env;

use ndarray::Array3;

use image::RgbImage;


use std::io;
use std::io::{
    Read, 
    BufReader
};
use std::fs::File;

fn read_bytes_from_file(file_name: &str) -> io::Result<Vec<u8>> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut buf = Vec::<u8>::new();

    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn save_array_as_img(array: Array3<u8>, file_name: &str) {
    let image = array_to_image(array);
    image.save(file_name).unwrap();
}

fn main() {
    let mut args = env::args().skip(1);
    // getting argument
    let input_file_name = args.next().unwrap().parse::<String>().unwrap();
    let output_file_name = args.next().unwrap().parse::<String>().unwrap();

    const width: usize = 256;
    const height: usize = 256;

    let bytes = read_bytes_from_file(&input_file_name);

    if let Err(e) = bytes {
        println!("Error loading bytes. error = {}", e);
        std::process::exit(1);
    }

    let bytes = bytes.unwrap();


    // processing
    let mut array = [0u16; width * height];
    for pair in bytes.windows(2) {
        let (x,y) = (pair[0] as usize, pair[1] as usize);
        array[x*width + y] += 1;
    }
    let array = array.iter().map(|v| (*v as f64).log(2.0)).collect::<Vec<_>>();

    // normalize the value
    let max = array.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    let array = array.iter().map(|v| (*v as f64) / (*max as f64)).collect::<Vec<f64>>();

    // convert it to 3d.
    let mut array3d: Array3<u8> = Array3::zeros((width, height, 3)); // width x height RGB
    for ((x, y, z), v) in array3d.indexed_iter_mut() {
        let x = x as usize;
        let y = y as usize;
        *v = (array[x*width + y] * 255.0).round() as u8
    }

    // save
    save_array_as_img(array3d, &output_file_name);
}
