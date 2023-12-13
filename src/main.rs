use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::process::Command;
use image::{GenericImageView, Rgba};
use std::thread;
use std::time::Duration;

fn create_dir_if_not_exist(dir: &Path) -> Result<(), String> {
    if !fs::metadata(dir).is_ok() {
        if let Err(err) = fs::create_dir(dir) {
            eprintln!("ERROR: could not create directory {}", err);
            return Err(format!("ERROR: could not create directory {err}"));
        }
        else{
            return Ok(());
        }
    }
    else {
        return Ok(());
    }
}

fn pixel_to_ascii(pixel: &Rgba<u8>) -> char {
    let gray_value = pixel[0] as f32*0.299 + pixel[1] as f32*0.587 + pixel[2] as f32*0.114;
    let ascii_chars = "@%#*+=-:. ";

    return ascii_chars.chars().nth((gray_value/255.0*9.0) as usize).unwrap_or(' ');
}

fn print_frame(frame: &Path) {
    let frame_image = image::open(frame).expect("Failed to open Frame.");
    let smaller_frame_image = frame_image.resize_exact(80,45, image::imageops::FilterType::Nearest);
    let ascii_art: String = smaller_frame_image.pixels()
        .map(|(_, _, pixel)| pixel_to_ascii(&pixel))
        .collect();

    for line in ascii_art.chars().collect::<Vec<char>>().chunks(80) {
        let line: String = line.iter().collect();
        println!("{}", line);
    }
}

fn get_frames(video: &Path) -> Vec<PathBuf> {
    let mut frames: Vec<PathBuf> = Vec::new();
    match create_dir_if_not_exist(Path::new("frames")) {
        Ok(()) => {
            let _ffmpeg_command = Command::new("ffmpeg")
                .arg("-i")
                .arg(video)
                .arg("frames/frame_%06d.png")
                .output();
            
            if let Ok(files) = fs::read_dir(Path::new("frames")) {
                for file in files {
                    if let Ok(file) = file {
                        let path = file.path();
                        frames.push(path);
                    }
                }
            }
            return frames;
        },
        Err(_) => todo!()
    }
}



fn play_frames(fps: u32, frames: Vec<PathBuf>) {
    let frame_time = Duration::from_secs_f64(1.0 / f64::from(fps));

    for frame in frames {
        print_frame(&frame);
        // thread::sleep(frame_time);
    }
}

fn main() {
    let mut frames: Vec<PathBuf> = get_frames(Path::new("/home/dylan/Seafile/My Library/Temp_Git_Files/vid2ascii/Shrek.mp4"));
    frames.sort();
    play_frames(30, frames);
    // print_frame(Path::new("/home/dylan/Seafile/My Library/Temp_Git_Files/vid2ascii/frames/frame_000446.png"));
}
