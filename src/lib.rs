#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate image;
extern crate num_complex;
extern crate wasm_bindgen;

use image::ImageBuffer;
use num_complex::Complex;
use std::f32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    // #[wasm_bindgen(method)]
    // fn getElementById(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn do_alert(item: &str) {
    alert(&format!("This is from WASM and {}", item));
}

#[wasm_bindgen]
pub struct FractalFactory {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl FractalFactory {
    #[wasm_bindgen]
    pub fn new(iterations: u16) -> Self {
        let color_pallete = [
            [66, 30, 15, 255],
            [25, 7, 26, 255],
            [9, 1, 47, 255],
            [4, 4, 73, 255],
            [0, 7, 100, 255],
            [12, 44, 138, 255],
            [24, 82, 177, 255],
            [57, 125, 209, 255],
            [134, 181, 229, 255],
            [211, 236, 248, 255],
            [241, 233, 191, 255],
            [248, 201, 95, 255],
            [255, 170, 0, 255],
            [204, 128, 0, 255],
            [153, 87, 0, 255],
            [106, 52, 3, 255],
        ] as [[u8; 4]; 16];

        let max_iterations = iterations;

        // let slider = document.getElementById("iterations-slider");
        // let _iter_value = slider.value();

        // let _v = kawk();

        let imgx = 720;
        let imgy = 720;

        let scalex = 2.8 / imgx as f32;
        let scaley = 2.8 / imgy as f32;

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = ImageBuffer::new(imgx, imgy);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let cy = y as f32 * scaley - 1.4;
            let cx = x as f32 * scalex - 1.4;

            let mut z = Complex::new(cx, cy);
            let c = Complex::new(-0.24, 0.67);

            let mut i = 0;

            for t in 0..max_iterations {
                if z.norm() > 6.0 {
                    break;
                }
                z = z * z + c;
                i = t;
            }

            *pixel = image::Rgba(color_pallete[(i as usize + 1) % color_pallete.len()]);
        }

        // Obtain the image's width and height.
        let (width, height) = imgbuf.dimensions();
        let pixels = imgbuf.into_vec();

        FractalFactory {
            pixels,
            height,
            width,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }
}

#[wasm_bindgen]
pub fn create_dom_elements() {
    let div = document.createElement("div");
    let p = document.createElement("p");

    p.set_inner_html("Hello from WASM in Rust.");

    div.append_child(p);

    document.body().append_child(div);
}
