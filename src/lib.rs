#![allow(clippy::unused_unit)]

use std::{collections::HashMap, io::Cursor};

use png::ColorType;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let offset_x = 0;
    let offset_y = 0;

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Initialize color friendly info
    let colors = vec![
        ColorOption {
            name: "dark red".to_string(),
            color: [190, 0, 57],
        },
        ColorOption {
            name: "red".to_string(),
            color: [16, 69, 0],
        },
        ColorOption {
            name: "orange".to_string(),
            color: [255, 168, 0],
        },
        ColorOption {
            name: "yellow".to_string(),
            color: [255, 214, 53],
        },
        ColorOption {
            name: "dark green".to_string(),
            color: [0, 163, 104],
        },
        ColorOption {
            name: "green".to_string(),
            color: [0, 204, 120],
        },
        ColorOption {
            name: "light green".to_string(),
            color: [126, 237, 86],
        },
        ColorOption {
            name: "dark teal".to_string(),
            color: [0, 117, 111],
        },
        ColorOption {
            name: "teal".to_string(),
            color: [0, 158, 170],
        },
        ColorOption {
            name: "dark blue".to_string(),
            color: [36, 80, 164],
        },
        ColorOption {
            name: "blue".to_string(),
            color: [54, 144, 234],
        },
        ColorOption {
            name: "light blue".to_string(),
            color: [81, 233, 244],
        },
        ColorOption {
            name: "indigo".to_string(),
            color: [73, 58, 193],
        },
        ColorOption {
            name: "periwinkle".to_string(),
            color: [106, 92, 255],
        },
        ColorOption {
            name: "dark purple".to_string(),
            color: [129, 30, 159],
        },
        ColorOption {
            name: "purple".to_string(),
            color: [180, 74, 192],
        },
        ColorOption {
            name: "pink".to_string(),
            color: [255, 56, 129],
        },
        ColorOption {
            name: "light pink".to_string(),
            color: [255, 153, 170],
        },
        ColorOption {
            name: "dark brown".to_string(),
            color: [109, 72, 47],
        },
        ColorOption {
            name: "brown".to_string(),
            color: [156, 105, 38],
        },
        ColorOption {
            name: "black".to_string(),
            color: [0, 0, 0],
        },
        ColorOption {
            name: "gray".to_string(),
            color: [137, 141, 144],
        },
        ColorOption {
            name: "light gray".to_string(),
            color: [212, 215, 217],
        },
        ColorOption {
            name: "white".to_string(),
            color: [255, 255, 255],
        },
    ];

    // Create a lookup map for image decoding
    let mut lookup = HashMap::new();
    for (i, color) in colors.iter().enumerate() {
        lookup.insert(color.color, i);
    }

    // Load the image from bundled data
    let image_data = include_bytes!("../docs/image.png");
    let decoder = png::Decoder::new(Cursor::new(image_data));
    let mut reader = decoder.read_info().unwrap();

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    assert_eq!(info.color_type, ColorType::Rgba);

    // Decode the image
    println!("Decoding image...");
    let mut cells = vec![0u8; (info.width * info.height) as usize];
    let (width, height) = (info.width as usize, info.height as usize);
    for (i, cell) in cells.iter_mut().enumerate() {
        let x = i as u32 % width as u32;
        let y = i as u32 / width as u32;

        let start_byte = i * 4;
        let pixel = &bytes[start_byte..start_byte + 4];
        let color = [pixel[0], pixel[1], pixel[2]];

        println!("{} {} {:?}", x, y, color);

        let color_index = lookup.get(&color).expect("Failed to look up color");
        *cell = *color_index as u8;
    }

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

struct ColorOption {
    name: String,
    color: [u8; 3],
}
