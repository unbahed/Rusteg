mod lib;
use std::collections::HashMap;

fn main() {
    let chunks = HashMap::from([(2, 16), (4, 4), (8, 2)]);
    let mut hash: HashMap<i32, u8> = HashMap::new();
    let (pixels, format) = lib::image_handler::read_image("cat.jpg".to_string());
    let full_len = pixels.len() * pixels[0].len();

    let text = lib::text_handler::read_file("t.txt".to_string());

    let chunk_in = 2;

    let text_byte = text.as_bytes();

    for i in 0..text.len() {
        let text_chr = text_byte[i];
        let first: u8 = text_chr / 16;
        let second: u8 = text_chr - (first * 16);

        hash.insert(i as i32, first, second);
    }

    println!("{:?}", text_byte);
    println!("{:?}", hash);
}
