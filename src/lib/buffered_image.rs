use crate::color::Color;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;

pub struct BufferedImage {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl BufferedImage {
    pub fn new(width: usize, height: usize) -> BufferedImage {
        BufferedImage {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize],
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

impl<I: SliceIndex<[Color]>> Index<I> for BufferedImage {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.pixels[index]
    }
}

impl<I: SliceIndex<[Color]>> IndexMut<I> for BufferedImage {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}
