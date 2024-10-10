use image::{ImageBuffer, Rgba};
use std::env;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ani2png-rs /path.ani");
        return;
    }

    read_file(&args[1]);
}

fn test_string(buffer: &[u8], start: usize) -> bool {
    if buffer.len() < start + 4 {
        return false;
    }
    buffer[start..start + 4] == [0x69, 0x63, 0x6f, 0x6e]
}

fn read_file(name: &str) {
    if !name.ends_with(".ani") {
        eprintln!("Usage: ani2png-rs /path.ani");
        return;
    }
    let path = Path::new(name);
    let file_stem = path.file_stem().unwrap().to_str().unwrap();
    let parent_dir = path.parent().unwrap_or(Path::new(""));
    let sub_dir = parent_dir.join(file_stem);
    if let Err(e) = fs::create_dir_all(&sub_dir) {
        eprintln!("Failed to create directory {}: {}", sub_dir.display(), e);
        return;
    }

    let mut file = match File::open(name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Unable to open file {}.", name);
            return;
        }
    };

    let file_len = file.seek(SeekFrom::End(0)).unwrap() as usize;
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut buffer = vec![0u8; file_len];
    file.read_exact(&mut buffer).unwrap();

    let mut png_counter = 1;
    let mut i = 0;
    while i < file_len {
        if png_counter == 9999 {
            return;
        }
        if i + 4 <= file_len && test_string(&buffer, i) {
            let new_png_name = format!("{}/{:04}.png", sub_dir.display(), png_counter);
            png_counter += 1;

            let mut icon_data = Vec::new();
            let mut j = 8;
            while i + j + 4 <= file_len {
                if test_string(&buffer, i + j + 1) {
                    break;
                }
                if j == 10 {
                    icon_data.push(0x01);
                } else {
                    icon_data.push(buffer[i + j]);
                }
                j += 1;
            }
            if i + j < file_len {
                icon_data.push(buffer[i + j]);
            }
            if i + j + 3 <= file_len {
                icon_data.extend_from_slice(&buffer[i + j + 1..i + j + 3]);
            }

            if let Ok(ico) =
                image::load_from_memory_with_format(&icon_data, image::ImageFormat::Ico)
            {
                let rgba_image = ico.to_rgba8();
                if let Err(e) = rgba_image.save(&new_png_name) {
                    eprintln!("Failed to save PNG: {}", e);
                }
            } else {
                eprintln!("Failed to load ICO data for frame {}", png_counter - 1);
            }

            i += j;
        } else {
            i += 1;
        }
    }
}
