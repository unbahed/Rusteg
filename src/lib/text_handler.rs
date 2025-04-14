use std::fs::read_to_string;

pub fn read_file(file_path: String) -> String {
    let content = read_to_string(file_path).unwrap();
    return content;
}

pub fn string_to_bytes(txt: String, chunk: u8) -> Vec<Vec<u8>> {
    let length = txt.len() / 2;

    let bit: Vec<Vec<u8>> = vec![vec![]];

    return bit;
}
