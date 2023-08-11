use std::fs;

use html_parser::{Dom, Element, Node};

use crate::text::Text;

pub struct Content {
    pub texts: Vec<Text>,
}

impl Content {
    pub fn new(input_path: &str) -> Content {
        let mut content = Content { texts: vec![] };

        let html = fs::read_to_string(input_path)
            .unwrap()
            .replace('\r', "")
            .replace('\n', "<br>");

        let dom = Dom::parse(&html).expect("Invalid input.");
        dom.children
            .iter()
            .for_each(|node| content.parse_node(node, &Text::new()));
        return content;
    }

    fn parse_node(&mut self, node: &Node, parent_text: &Text) {
        let mut text = parent_text.clone();
        match node {
            Node::Text(string) => {
                text.content = string.to_string();
                self.texts.push(text);
            }
            Node::Element(element) => {
                self.parse_element(element, &mut text);
                element
                    .children
                    .iter()
                    .for_each(|node| self.parse_node(node, &text));
            }
            _ => (),
        }
    }

    fn parse_element(&mut self, element: &Element, text: &mut Text) {
        match element.name.as_str() {
            "font" => {
                text.parse_font(element);
            }
            _ => (),
        }
    }
}
