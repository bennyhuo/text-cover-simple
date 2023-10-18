use std::any::type_name;
use std::fmt::Debug;
use std::str::FromStr;

use csscolorparser::Color;
use html_parser::Element;
use imageproc::drawing::text_size;
use rusttype::{Font, Scale};

#[derive(Clone, Debug)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub font_color: Color,
    pub background_color: Color,
}

impl Text {
    pub fn new() -> Text {
        Text {
            content: "".to_string(),
            font_size: 120f32,
            font_color: Color::from([0, 0, 0, 255]),
            background_color: Color::from([255, 255, 255, 255]),
        }
    }

    pub fn scale(&self) -> Scale {
        Scale::uniform(self.font_size)
    }

    pub fn size(&self, font: &Font) -> (u32, u32) {
        let (w, h) = text_size(self.scale(), &font, self.content.replace(" ", "0").as_str());
        (w as u32, h as u32)
    }

    pub fn parse_font(&mut self, element: &Element) {
        Text::parse_attribute(element, &mut self.font_size, "size");
        Text::parse_attribute(element, &mut self.font_color, "color");
        Text::parse_attribute(element, &mut self.background_color, "background");
    }

    fn parse_attribute<U: FromStr>(element: &Element, u: &mut U, key: &str)
    where
        U::Err: Debug,
    {
        let option = element.attributes.get(key).and_then(|value| {
            value.as_ref().map(|value| {
                value
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid {}. {} expected.", key, type_name::<U>()))
            })
        });

        if let Some(value) = option {
            *u = value;
        }
    }
}
