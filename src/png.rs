use std::io::Write;

use anyhow::Context;
use png::{AdaptiveFilterType, BitDepth, ColorType, Compression, Encoder};

use crate::luma4::Luma4Image;

pub fn png_save<W: Write>(w: W, image: Luma4Image) -> anyhow::Result<()> {
    let mut encoder = Encoder::new(w, image.width, image.height);
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(BitDepth::Four);
    encoder.set_compression(Compression::Best);
    encoder.set_adaptive_filter(AdaptiveFilterType::Adaptive);
    let mut writer = encoder.write_header().context("Failed to write header")?;
    writer
        .write_image_data(&image.bytes)
        .context("Failed to write an image")?;
    Ok(())
}
