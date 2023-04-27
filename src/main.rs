use image::{imageops};
use image::imageops::FilterType;
use std::fs::{read_dir, create_dir_all};
use std::path::{PathBuf};
use rayon::prelude::*;
use png::Compression;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let input_folder = current_dir.to_str().unwrap();

    let converted_folder = format!("{}/converted", input_folder);
    create_dir_all(&converted_folder).unwrap();

    let files: Vec<_> = read_dir(input_folder).unwrap().map(|f| f.unwrap()).collect();

    files.par_iter().for_each(|entry| {
        let path = entry.path();
        println!("Processing file {:?}", path);

        if path.is_file() {
            let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default();

            if extension == "jpg" || extension == "jpeg" || extension == "png" {
                let image_to_convert = image::open(&path).unwrap().to_rgba8();
                let (width, height) = image_to_convert.dimensions();

                let new_width = if width > height { 512 } else { width * 512 / height };
                let new_height = if height > width { 512 } else { height * 512 / width };

                let resized_image = imageops::resize(&image_to_convert, new_width, new_height, FilterType::Gaussian);

                let new_file_name = format!(
                    "{}+CONVERTED.png",
                    path.file_stem().unwrap().to_str().unwrap(),
                );

                let new_file_path = PathBuf::from(&converted_folder).join(&new_file_name);

                let mut png_encoder = png::Encoder::new(
                    std::fs::File::create(&new_file_path).unwrap(),
                    resized_image.width(),
                    resized_image.height(),
                );
                png_encoder.set_color(png::ColorType::Rgba);
                png_encoder.set_depth(png::BitDepth::Eight);
                png_encoder.set_compression(Compression::Best);
                png_encoder.set_filter(png::FilterType::Avg);

                let mut png_writer = png_encoder.write_header().unwrap();

                let mut png_data: Vec<u8> = vec![0; (4 * resized_image.width() * resized_image.height()) as usize];
                for (i, pixel) in resized_image.pixels().enumerate() {
                    png_data[4 * i] = pixel[0];
                    png_data[4 * i + 1] = pixel[1];
                    png_data[4 * i + 2] = pixel[2];
                    png_data[4 * i + 3] = pixel[3];
                }

                png_writer.write_image_data(&png_data).unwrap();
            }
        }
    });
}
