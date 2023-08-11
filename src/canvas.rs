use csscolorparser::Color;
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;

use crate::content::Content;
use crate::text::Text;
use image::{ImageBuffer, Rgba};
use rusttype::Font;

pub trait Drawable {
    fn fill_color(&mut self, color: Color);

    fn draw(&mut self, content: &Content, rect: Rect, font: &Font);

    fn draw_text(&mut self, text: &Text, rect: &mut Rect, font: &Font);
}

pub type ImageCanvas = ImageBuffer<Rgba<u8>, Vec<u8>>;

impl Drawable for ImageCanvas {
    fn fill_color(&mut self, color: Color) {
        draw_filled_rect_mut(
            self,
            Rect::at(0, 0).of_size(self.width(), self.height()),
            Rgba(color.to_rgba8()),
        );
    }

    fn draw(&mut self, content: &Content, content_rect: Rect, font: &Font) {
        if content.texts.is_empty() {
            return;
        }

        let content_height = content
            .texts
            .iter()
            .map(|text| text.size(font).1)
            .max()
            .unwrap_or(0);

        let mut rect = Rect::at(
            content_rect.left(),
            content_rect.top() + ((content_rect.height() - content_height) / 2) as i32,
        )
        .of_size(content_rect.width(), content_height);

        content.texts.iter().for_each(|text| {
            self.draw_text(text, &mut rect, font);
        });
    }

    fn draw_text(&mut self, text: &Text, rect: &mut Rect, font: &Font) {
        let (text_width, text_height) = text.size(font);
        let metrics = font.v_metrics(text.scale());

        let offset_y = (rect.height() - metrics.ascent.round() as u32) as i32;

        let text_rect =
            Rect::at(rect.left(), rect.top() + offset_y).of_size(text_width, text_height);

        *rect = Rect::at(rect.left() + text_width as i32, rect.top())
            .of_size(rect.width() - text_width, rect.height());

        draw_filled_rect_mut(self, text_rect, Rgba(text.background_color.to_rgba8()));

        draw_text_mut(
            self,
            Rgba(text.font_color.to_rgba8()),
            text_rect.left(),
            text_rect.top(),
            text.scale(),
            &font,
            text.content.as_str(),
        );
    }
}
