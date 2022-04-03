#![allow(clippy::unused_unit)]

mod image;
mod palette;

use std::collections::HashMap;

use rand::Rng;
use wasm_bindgen::prelude::*;

use crate::{image::load_image_cells, palette::create_palette};

#[wasm_bindgen(start)]
pub fn main() {
    let offset_x = 0;
    let offset_y = 0;

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let colors = create_palette();

    // Create a lookup map for image decoding
    let mut lookup = HashMap::new();
    for (i, color) in colors.iter().enumerate() {
        lookup.insert(color.color, i);
    }

    let (width, height, cells) = load_image_cells(&lookup);

    // Pick the random pixel
    let index = rand::thread_rng().gen_range(1..(cells.len()));
    let relative_x = index % width;
    let relative_y = index / height;
    let x = relative_x + offset_x;
    let y = relative_y + offset_y;

    let color = &colors[cells[index] as usize];
    let text = format!("Your pixel is {} at {}, {}!", color.name, x, y);

    // Initialize the page with the picked color
    let val = document.get_element_by_id("app").unwrap();
    val.set_inner_html(&text);
}
