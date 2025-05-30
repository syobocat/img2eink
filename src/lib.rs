use std::path::Path;

use image::{DynamicImage, imageops};
use luma4::Luma4Image;

mod luma4;
mod png;

fn process_image(mut image: DynamicImage, width: u32, height: u32) -> Luma4Image {
    if image.width() > width || image.height() > height {
        image = image.resize(width, height, imageops::FilterType::Lanczos3);
    }
    image.into()
}

pub fn process_and_save_image(
    image: DynamicImage,
    dimention: (u32, u32),
    path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let processed = process_image(image, dimention.0, dimention.1);
    processed.save(path)
}
