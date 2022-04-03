#![allow(clippy::unused_unit)]

mod image;
mod palette;

use std::collections::HashMap;

use once_cell::sync::Lazy;
use palette::PaletteColor;
use rand::Rng;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;

use crate::{image::load_image_cells, palette::create_palette};

#[wasm_bindgen(start)]
pub fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Set up the button
    let button = document
        .get_element_by_id("pp-button-new")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let click_handler = Closure::wrap(Box::new(move || {
        pick_new_pixel();
    }) as Box<dyn FnMut()>);
    button.set_onclick(Some(click_handler.as_ref().unchecked_ref()));
    click_handler.forget();

    // Initialize overview data
    let topleft_text = format!("{}, {}", GLOBAL.offset_x, GLOBAL.offset_y);
    let label = document.get_element_by_id("pp-label-topleft").unwrap();
    label.set_inner_html(&topleft_text);

    // Pick pixel automatically
    pick_new_pixel();
}

fn pick_new_pixel() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Pick the random pixel
    let index = rand::thread_rng().gen_range(1..(GLOBAL.cells.len()));
    let relative_x = index % GLOBAL.width;
    let relative_y = index / GLOBAL.height;
    let x = relative_x + GLOBAL.offset_x;
    let y = relative_y + GLOBAL.offset_y;

    let color = &GLOBAL.colors[GLOBAL.cells[index] as usize];
    let text = format!("Your pixel is {} at {}, {}!", color.name, x, y);

    let color_str = format!(
        "rgb({}, {}, {})",
        color.color[0], color.color[1], color.color[2]
    );

    // Initialize the page with the picked color
    let label = document.get_element_by_id("pp-label-assigned").unwrap();
    label.set_inner_html(&text);

    let color_box = document
        .get_element_by_id("pp-color")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    color_box
        .style()
        .set_property("background-color", &color_str)
        .unwrap();
}

static GLOBAL: Lazy<GlobalData> = Lazy::new(|| {
    let colors = create_palette();

    // Create a lookup map for image decoding
    let mut lookup = HashMap::new();
    for (i, color) in colors.iter().enumerate() {
        lookup.insert(color.color, i);
    }

    let (width, height, cells) = load_image_cells(&lookup);

    GlobalData {
        colors,
        width,
        height,
        offset_x: 0,
        offset_y: 0,
        cells,
    }
});

struct GlobalData {
    colors: Vec<PaletteColor>,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    cells: Vec<u8>,
}
