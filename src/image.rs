use std::{collections::HashMap, io::Cursor};

use png::ColorType;

pub fn load_image_cells(lookup: &HashMap<[u8; 3], usize>) -> (usize, usize, Vec<u8>) {
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

    (width, height, cells)
}
