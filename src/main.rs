use std::path::PathBuf;

use clap::{ArgAction, Parser};
use image::ImageReader;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct App {
    /// Show this help
    #[arg(long, action = ArgAction::Help)]
    help: bool,

    /// Traverse the directory recursively
    #[arg(short, long)]
    recurse: bool,

    /// File or directory to write converted image to
    #[arg(short, long)]
    dest: Option<PathBuf>,

    /// Screen width of your E-ink device
    #[arg(short, long)]
    width: u32,

    /// Screen height of your E-ink device
    #[arg(short, long)]
    height: u32,

    /// File or directory to convert
    target: PathBuf,
}

fn main() {
    let app = App::parse();

    if app.target.is_dir() {
        let dest = app.dest.unwrap_or_else(|| {
            let dir = app.target.parent().unwrap();
            let name = app.target.file_stem().unwrap().to_string_lossy();
            dir.join(format!("{name}_eink"))
        });
        process_dir(app.target, dest, (app.width, app.height), app.recurse);
    } else {
        let image = ImageReader::open(&app.target)
            .expect("Target should be accessible")
            .decode()
            .expect("Target should be a valid image");
        let dest = if let Some(dest) = app.dest {
            if dest.is_file() {
                dest
            } else {
                let name = app.target.file_stem().unwrap().to_string_lossy();
                dest.join(format!("{name}.png"))
            }
        } else {
            let dir = app.target.parent().unwrap();
            let name = app.target.file_stem().unwrap().to_string_lossy();
            dir.join(format!("{name}_eink.png"))
        };
        img2eink::process_and_save_image(image, (app.width, app.height), dest).unwrap();
    }
}

fn process_dir(dir: PathBuf, dest: PathBuf, dimention: (u32, u32), recursive: bool) {
    std::fs::create_dir_all(&dest).expect("Destination should be writable");
    let Ok(mut children) = std::fs::read_dir(dir) else {
        return;
    };
    while let Some(Ok(child)) = children.next() {
        let path = child.path();
        if path.is_dir() {
            if recursive {
                let next_dest = dest.join(path.file_name().unwrap());
                process_dir(path, next_dest, dimention, recursive);
            }
        } else {
            let Ok(image) = ImageReader::open(&path) else {
                eprintln!("Failed to open: {path:?}");
                continue;
            };
            let Ok(image) = image.decode() else {
                eprintln!("Failed to decode: {path:?}");
                continue;
            };
            let name = path.file_stem().unwrap().to_string_lossy();
            let dest = dest.join(format!("{name}.png"));
            if img2eink::process_and_save_image(image, dimention, dest).is_err() {
                eprintln!("Failed to process: {path:?}");
                continue;
            };
        }
    }
}
