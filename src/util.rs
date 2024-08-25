//! Utilities related to the dialog widget display

use bevy_terminal_display::ratatui::style::{Color, Style, Stylize};
use yarnspinner::runtime::MarkupValue;
use zalgo::{Generator, GeneratorArgs, ZalgoSize};

/// Splits a marked-up yarnspinner line into sections of text and their corresponding ratatui styles
pub fn style_line(line: &yarnspinner::runtime::Line) -> Vec<(String, Style)> {
    if line.attributes.is_empty() {
        return vec![(line.text_without_character_name(), Style::new())];
    }

    let mut line_segments = Vec::<(String, Style)>::new();
    let mut attributes = line.attributes.clone();
    attributes.sort_by_key(|attribute| attribute.position);
    line_segments.push((
        line.text[..attributes[0].position].to_string(),
        Style::new(),
    ));
    for (i, attribute) in attributes.iter().enumerate() {
        let mut attrib_text = line.text_for_attribute(&attribute).to_string();
        let mut style = Style::new();
        match attribute.name.as_str() {
            "style" => {
                for (property_name, property_value) in attribute.properties.iter() {
                    match property_name.as_str() {
                        "bold" => {
                            if let MarkupValue::Bool(value) = property_value {
                                if *value {
                                    style = style.bold();
                                }
                            }
                        }
                        "italic" => {
                            if let MarkupValue::Bool(value) = property_value {
                                if *value {
                                    style = style.italic();
                                }
                            }
                        }
                        "color" => {
                            if let MarkupValue::Integer(value) = property_value {
                                style = style.fg(Color::Indexed(*value as u8))
                            }
                        }
                        "zalgo" => {
                            if let MarkupValue::Bool(value) = property_value {
                                if *value {
                                    let mut generator = Generator::new();
                                    let mut out = String::new();
                                    let args =
                                        GeneratorArgs::new(true, true, true, ZalgoSize::Mini);
                                    generator.gen(&attrib_text, &mut out, &args);
                                    attrib_text = out;
                                }
                            }
                        }
                        "bg" => {
                            if let MarkupValue::Integer(value) = property_value {
                                style = style.bg(Color::Indexed(*value as u8))
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        if attribute.name != "character" {
            line_segments.push((attrib_text, style))
        }

        if let Some(next_attribute) = attributes.get(i + 1) {
            line_segments.push((
                line.text[attribute.position + attribute.length..next_attribute.position]
                    .to_string(),
                Style::new(),
            ));
        }
    }
    let last_attribute = attributes.last().unwrap();
    line_segments.push((
        line.text[last_attribute.position + last_attribute.length..].to_string(),
        Style::new(),
    ));

    line_segments
}
