use image::{imageops, GenericImageView};
use image::imageops::FilterType;
use std::fs::{read_dir, create_dir_all};
use std::path::{Path, PathBuf};
use rayon::prelude::*;

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
                let image_to_convert = image::open(&path).unwrap();
                let (width, height) = image_to_convert.dimensions();

                let new_width = if width > height { 512 } else { width * 512 / height };
                let new_height = if height > width { 512 } else { height * 512 / width };

                let resized_image = imageops::resize(&image_to_convert, new_width, new_height, FilterType::Lanczos3);

                let new_file_name = format!(
                    "{}+CONVERTED.png",
                    path.file_stem().unwrap().to_str().unwrap(),
                );

                let new_file_path = PathBuf::from(&converted_folder).join(&new_file_name);

                resized_image.save(&new_file_path).unwrap();
            }
        }
    });
}

