use image::{ImageFormat, ImageReader, Rgba};

pub fn read_image(image_path: String) -> (Vec<Vec<Rgba<u8>>>, ImageFormat) {
    let img = ImageReader::open(image_path.clone()).unwrap();
    let format = &img.format().unwrap();
    let img_decoded = img.decode().unwrap().to_rgba8();

    let mut img_pix = img_decoded.pixels();
    let width = usize::try_from(img_decoded.width()).unwrap();
    let height = usize::try_from(img_decoded.height()).unwrap();

    let mut px: Vec<Vec<Rgba<u8>>> = vec![vec![Rgba([0; 4]); height]; width];

    let mut x = 0;
    let mut y = 0;

    for p in 0..img_pix.len() {
        let px_iter = img_pix.next().unwrap();
        if x >= width {
            x = 0;
            y += 1;
            px[x][y] = px_iter.clone();
        } else {
            px[x][y] = px_iter.clone();
            x += 1;
        }
    }

    return (px, format.clone());
}
