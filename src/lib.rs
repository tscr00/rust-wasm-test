#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

struct Point2d {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

trait Canvas {
    fn draw_point(&mut self, point: Point2d, color: Color);
}

#[wasm_bindgen]
pub struct FlatCanvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Color {
    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0, a: 255 }
    }
    pub fn white() -> Color {
        Color { r: 255, g: 255, b: 255, a: 255 }
    }
}

#[wasm_bindgen]
impl FlatCanvas {

    pub fn new(width: usize, height: usize) -> FlatCanvas {
        let pixels = vec![Color::white(); width*height];
        FlatCanvas { width, height, pixels }
    }

    pub fn canvas(&self) -> *const Color {
        self.pixels.as_ptr()
    }
}
