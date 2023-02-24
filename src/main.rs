use image::{
    png::{CompressionType, FilterType, PngEncoder},
    ColorType, GrayImage, Luma,
};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

// the dimensions of the image
const IMG_WIDTH: u32 = 1280;
const IMG_HEIGHT: u32 = 720;

// get the binary string from the file
fn get_bin(filename: &String) -> String {
    let mut f = File::open(&filename).expect("Invalid file path");
    let metadata = fs::metadata(&filename).expect("Unable to read the file");

    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer Overflowed");

    // convert this buffer to binary string
    let binary_string = buffer
        .iter()
        .map(|b| format!("{:b}", b))
        .collect::<Vec<String>>()
        .join("");

    binary_string
}

fn main() -> Result<(), Box<dyn Error>> {
    let s = "./raw/input.pdf";
    let _v = get_bin(&s.to_string());

    // input and output file for the image
    let input_file = File::open("./raw/input.txt").expect("Failed to open the file.");
    let reader = BufReader::new(input_file);
    let mut output_file = File::create("./raw/output.bin").expect("Failed to create the file.");

    // the output single image
    let mut output_img = GrayImage::new(IMG_WIDTH as u32, IMG_HEIGHT as u32);
    let black_color = [255 as u8];
    let white_color = [0 as u8];

    // currently writing pixel in both x and y
    let mut x = 0;
    let mut y = 0;
    // let mut current_line = 1;
    let mut current_image = 1;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let binary_line = line
            .as_bytes()
            .iter()
            .map(|b| format!("{:b}", b))
            .collect::<Vec<String>>()
            .join("");

        // println!("{}", binary_line);
        output_file.write_all(binary_line.as_bytes())?;

        // itterate over all the binary numbers
        for bin in binary_line.chars() {
            if bin == '1' {
                output_img.put_pixel(x, y, Luma(white_color));
            } else {
                output_img.put_pixel(x, y, Luma(black_color));
            }

            // if the current pixel is the last one in the line
            if x == IMG_WIDTH - 1 {
                // if the current line is the last one in the image
                if y == IMG_HEIGHT - 1 {
                    // save the image
                    let image_name = format!("./raw/output_{}.png", current_image);
                    let encoder = PngEncoder::new_with_quality(
                        File::create(image_name).unwrap(),
                        CompressionType::Best,
                        FilterType::NoFilter,
                    );
                    encoder
                        .encode(
                            &mut output_img,
                            IMG_WIDTH as u32,
                            IMG_HEIGHT as u32,
                            ColorType::L8,
                        )
                        .unwrap();

                    output_img = GrayImage::new(IMG_WIDTH, IMG_HEIGHT);
                    current_image += 1;
                    x = 0;
                    y = 0;
                } else {
                    // reset the x
                    x = 0;
                    y += 1;
                }
            } else {
                x += 1;
            }
        }

        // current_line += 1;
    }

    // convert all the images to a video
    let mut command = Command::new("ffmpeg");
    command
        .arg("-framerate")
        .arg("30")
        .arg("-i")
        .arg("./raw/output_%d.png")
        .arg("-c:v")
        .arg("libx264")
        .arg("-r")
        .arg("30")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("output.mp4");

    command.current_dir(Path::new("./"));
    let output = command.output().expect("Failed to execute command");
    println!("{:?}", output);

    let image_name = format!("./raw/output_{}.png", current_image);
    output_img.save(image_name).unwrap();

    Ok(())
}
