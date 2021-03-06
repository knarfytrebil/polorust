use handlebars::{handlebars_helper, Handlebars};
use serde_json::Value;
use treexml::{Document, Element};
use tui::text::{Span, Spans};
use tui::widgets::Block;
use tui::widgets::Paragraph;

pub enum El {
    Paragraph(Paragraph<'static>),
    Spans(Spans<'static>),
    Span(Span<'static>),
}

pub fn parse_xml(xml: String) -> Element {
    let doc = Document::parse(xml.as_bytes()).unwrap();
    doc.root.unwrap()
}

// fn generate_line_buffer(height: u16) -> Box<dyn Fn(Vec<Value>) -> Vec<Value>> {
//     handlebars_helper!(line_buffer: |v: Vec| {
//         let buffer_start = match height as usize <= lines.len() {
//             false => 0,
//             true => lines.len() - height as usize
//         };
//         (&lines[buffer_start..]).to_vec()
//
//     })
// }

pub fn parse(template: String, v: &Value) -> Element {
    let reg = Handlebars::new();
    // reg.register_helper("line_buffer", line_buffer(v["metrics"]["height"].as_u16()));

    let filled_template = reg
        .render_template(&template, &v)
        .expect("Template Parse Error");
    parse_xml(filled_template)
}

// fn inner_buffer(area_height: u16, lines: Vec<Value>) -> Vec<Value> {
//     let buffer_start = match area_height as usize <= lines.len() {
//         false => 0,
//         true => lines.len() - area_height as usize
//     };
//     (&lines[buffer_start..]).to_vec()
// }

pub fn create_element(el: Element) -> El {
    let children: Vec<El> = match !el.children.is_empty() {
        true => el.children.into_iter().map(create_element).collect(),
        false => vec![],
    };

    let styles: Option<Value> = match el.attributes.contains_key("styles") {
        true => Some(serde_json::from_str(&el.attributes["styles"]).expect("JSON Parse Error")),
        false => None,
    };

    debug!("STYLES: {:?}", styles);

    let this = match el.name.as_str() {
        "Paragraph" => {
            let el_list: Vec<Spans> = match !children.is_empty() {
                true => children
                    .into_iter()
                    .map(|child| match child {
                        El::Spans(s) => s,
                        _ => panic!("Not a Text Node!"),
                    })
                    .collect(),
                false => vec![],
            };
            El::Paragraph(Paragraph::new(el_list))
        }
        "Spans" => match !children.is_empty() {
            true => {
                let span_list: Vec<Span> = children
                    .into_iter()
                    .map(|child| match child {
                        El::Span(s) => s,
                        _ => panic!("Not a Text Node!"),
                    })
                    .collect();
                El::Spans(Spans::from(span_list))
            }
            false => {
                let text = match el.text {
                    Some(txt) => txt,
                    None => String::from(""),
                };
                El::Spans(Spans::from(text))
            }
        },
        "Span" => {
            let text = match el.text {
                Some(txt) => txt,
                None => String::from(""),
            };
            El::Span(Span::from(text))
        }
        &_ => panic!("Unknown DOM Token"),
    };

    this
}
