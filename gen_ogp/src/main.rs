use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

fn main() {
    let mut image = image::open("src/assets/image/pr01170.jpg").unwrap();
    let font = Vec::from(include_bytes!("src/assets/fonts/DelaGothicOne-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 300.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    let text = "万年金欠";
    draw_text_mut(&mut image, Rgba([225u8, 225u8, 225u8, 225u8]), 50, 0, scale, &font, text);

    image.save("sample.jpg").unwrap();
}
