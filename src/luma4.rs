use std::{fs::File, path::Path};

use anyhow::Context;
use image::{
    DynamicImage, Luma,
    imageops::{self, ColorMap},
};

use crate::png::png_save;

struct Luma4;
pub struct Luma4Image {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl ColorMap for Luma4 {
    type Color = Luma<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Self::Color) -> usize {
        let luma = color.0;
        luma[0] as usize / 16
    }

    #[inline(always)]
    fn lookup(&self, index: usize) -> Option<Self::Color> {
        match index {
            0..16 => Some([index as u8 * 16].into()),
            _ => None,
        }
    }

    #[inline(always)]
    fn has_lookup(&self) -> bool {
        true
    }

    #[inline(always)]
    fn map_color(&self, color: &mut Self::Color) {
        let new_color = 16 * self.index_of(color) as u8;
        let luma = &mut color.0;
        luma[0] = new_color;
    }
}

impl Luma4Image {
    pub fn save(self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let file = File::create(path).context("Failed to open file")?;
        png_save(file, self)
    }
}

impl From<DynamicImage> for Luma4Image {
    fn from(image: DynamicImage) -> Self {
        let mut luma8 = image.into_luma8();
        imageops::dither(&mut luma8, &Luma4);

        let width = luma8.width();
        let height = luma8.height();
        let bytes = luma8
            .chunks(2)
            .map(|bytes| bytes[1] >> 4 | bytes[0])
            .collect();

        Self {
            bytes,
            width,
            height,
        }
    }
}
