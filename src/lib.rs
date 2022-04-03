#![allow(clippy::unused_unit)]

mod image;
mod palette;

use std::collections::HashMap;

use once_cell::sync::Lazy;
use palette::PaletteColor;
use rand::Rng;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

use crate::{image::load_image_cells, palette::create_palette};

const READY: bool = true;
const OFFSET_X: usize = 879;
const OFFSET_Y: usize = 1898;

#[wasm_bindgen(start)]
pub fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    if READY {
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
        let topleft_text = format!("{}, {}", OFFSET_X, OFFSET_Y);
        let label = document.get_element_by_id("pp-label-topleft").unwrap();
        label.set_inner_html(&topleft_text);

        // Pick pixel automatically
        pick_new_pixel();
    } else {
        // Display not-ready
        let label = document.get_element_by_id("pp-label-assigned").unwrap();
        label.set_inner_html("Nothing yet! Follow the RubberRoss Twitch stream for instructions.");
    }
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
        cells,
    }
});

struct GlobalData {
    colors: Vec<PaletteColor>,
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

fn pick_new_pixel() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Pick the random pixel
    let index = rand::thread_rng().gen_range(1..(GLOBAL.cells.len()));
    let relative_x = index % GLOBAL.width;
    let relative_y = index / GLOBAL.height;
    let x = relative_x + OFFSET_X;
    let y = relative_y + OFFSET_Y;

    let color = &GLOBAL.colors[GLOBAL.cells[index] as usize];
    let text = format!("Your pixel is <span class=\"font-semibold\">{}</span> at <span class=\"font-semibold\">{}</span>, <span class=\"font-semibold\">{}</span>!", color.name, x, y);
    let direct_link = format!("https://www.reddit.com/r/place/?cx={}&cy={}&px=11", x, y);
    let link_html = format!("<a target=\"_blank\" rel=\"noopener noreferrer\" href=\"{}\">Direct link to r/place location...</a>", direct_link);

    let color_str = color_to_rgb(color.color);

    // Initialize the page with the picked color
    let label_asigned = document.get_element_by_id("pp-label-assigned").unwrap();
    label_asigned.set_inner_html(&text);
    let label_link = document.get_element_by_id("pp-label-directlink").unwrap();
    label_link.set_inner_html(&link_html);

    let color_box = document
        .get_element_by_id("pp-color")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    color_box
        .style()
        .set_property("background-color", &color_str)
        .unwrap();

    // Redraw the canvas with the picked pixel
    redraw_canvas(relative_x as u32, relative_y as u32);
}

fn redraw_canvas(pixel_x: u32, pixel_y: u32) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Fetch canvas and context from the DOM
    let canvas = document
        .get_element_by_id("pp-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let pixel_size = 5;

    // Get relative coordinates for centering the image
    let canvas_width = canvas.width() as u32;
    let canvas_height = canvas.height() as u32;
    let image_width = GLOBAL.width as u32 * pixel_size;
    let image_height = GLOBAL.height as u32 * pixel_size;
    let offset_x = (canvas_width - image_width) / 2;
    let offset_y = (canvas_height - image_height) / 2;

    // Clear the canvas before drawing the new content
    context.clear_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);

    // Draw the image centered on the screen
    for (i, cell) in GLOBAL.cells.iter().enumerate() {
        let x = (i % GLOBAL.width) as u32;
        let y = (i / GLOBAL.height) as u32;

        let fill = color_to_rgb(GLOBAL.colors[*cell as usize].color);
        context.set_fill_style(&fill.into());
        context.fill_rect(
            (offset_x + (x * pixel_size)) as f64,
            (offset_y + (y * pixel_size)) as f64,
            pixel_size as f64,
            pixel_size as f64,
        );
    }

    // Outline the picked pixel
    let pixel_canvax_x = offset_x + (pixel_x * pixel_size);
    let pixel_canvax_y = offset_y + (pixel_y * pixel_size);

    context.set_fill_style(&"rgba(255, 0, 0, 0.8)".into());

    // top
    context.fill_rect(
        (pixel_canvax_x - 2) as f64,
        (pixel_canvax_y - 2) as f64,
        (pixel_size + 4) as f64,
        2.0,
    );
    // bottom
    context.fill_rect(
        (pixel_canvax_x - 2) as f64,
        (pixel_canvax_y + pixel_size) as f64,
        (pixel_size + 4) as f64,
        2.0,
    );
    // left
    context.fill_rect(
        (pixel_canvax_x - 2) as f64,
        (pixel_canvax_y) as f64,
        2.0,
        (pixel_size) as f64,
    );
    // right
    context.fill_rect(
        (pixel_canvax_x + pixel_size) as f64,
        (pixel_canvax_y) as f64,
        2.0,
        (pixel_size) as f64,
    );
}

fn color_to_rgb(color: [u8; 3]) -> String {
    format!("rgb({}, {}, {})", color[0], color[1], color[2])
}
