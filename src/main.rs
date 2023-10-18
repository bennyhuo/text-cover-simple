use std::fs;

use clap::Parser;
use csscolorparser::Color;
use imageproc::rect::Rect;
use rusttype::Font;

use crate::canvas::{Drawable, ImageCanvas};
use crate::content::Content;

mod canvas;
mod content;
mod text;

#[derive(Parser, Debug)]
pub struct Parameter {
    #[arg(long, default_value_t = 1920)]
    pub image_width: u32,
    #[arg(long, default_value_t = 1080)]
    pub image_height: u32,
    #[arg(long, default_value_t = 100)]
    pub padding: u32,
    #[arg(long, default_value = "#FFFFFFFF")]
    pub background_color: Color,
    #[arg(short, long)]
    pub input_path: String,
    #[arg(short, long)]
    pub output_path: String,
}

fn main() {
    let params = Parameter::parse();
    let font = load_font("assets/JetBrainsMono-Regular.ttf");
    let content = Content::new(&params.input_path);
    dbg!(&content);

    let mut canvas = ImageCanvas::new(params.image_width, params.image_height);
    canvas.fill_color(params.background_color);
    canvas.draw(
        &content,
        Rect::at(params.padding as i32, params.padding as i32).of_size(
            params.image_width - params.padding * 2,
            params.image_height - params.padding * 2,
        ),
        &font,
    );
    canvas.save(&params.output_path).unwrap();
}

pub fn load_font(path: &str) -> Font {
    let bytes = fs::read(path).expect("File not found");
    Font::try_from_vec(bytes).expect("Failed to create font. Invalid font data.")
}
