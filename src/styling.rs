// use quick_xml::{events::Event, reader::Reader};
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};
use std::collections::VecDeque;

pub fn html_to_ratatui(mut html: &[u8]) -> Text {
    let dom = html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html)
        .unwrap();

    fn dom_to_ratatui(node: &Handle, parent_style: Style) -> Text<'static> {
        match &node.data {
            NodeData::Text { contents } => {
                let s: String = contents.borrow().to_string();
                let mut text = Text::default();
                text.push_line(Line::from(Span::styled(s, parent_style)));
                text
                // let mut text = Text::styled("", parent_style);
                // for word in s.split_whitespace() {
                //     text.push_span(Span::styled(word.to_string(), parent_style));
                //     text.push_span(Span::styled(" ", parent_style));
                // }
                // text
            }
            NodeData::Element { name, .. } => {
                let mut text = Text::default();
                let mut style = parent_style;
                match name.local.as_ref() {
                    "h1" => {
                        style = style
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::UNDERLINED);
                    }
                    "b" | "strong" => {
                        style = style.add_modifier(Modifier::BOLD);
                    }
                    "em" => {
                        style = style.add_modifier(Modifier::ITALIC);
                    }
                    "a" => {
                        style = style.add_modifier(Modifier::UNDERLINED).fg(Color::Blue);
                    }
                    _ => {}
                }
                for child in &node.children.clone().into_inner() {
                    let is_block = if let NodeData::Element { name, .. } = &child.data {
                        matches!(
                            name.local.as_ref(),
                            "h1" | "p" | "br" | "main" | "div" | "html" | "body"
                        )
                    } else {
                        false
                    };

                    let child_text = dom_to_ratatui(child, style);

                    if child_text.lines.is_empty() {
                        continue;
                    };

                    if is_block {
                        text.push_line(Line::default());
                        for line in child_text.lines {
                            text.push_line(line);
                        }
                    } else {
                        if text.lines.is_empty() {
                            text.push_line(Line::default())
                        }
                        for line in child_text.lines {
                            for span in line.spans {
                                text.push_span(span);
                            }
                        }
                    }
                }
                text
            }
            NodeData::Document => {
                let mut text = Text::default();
                for child in &node.children.clone().into_inner() {
                    let child_text = dom_to_ratatui(child, parent_style);
                    for line in child_text.lines {
                        text.push_line(line)
                    }
                }
                text
            }
            _ => Text::raw("Could not read"),
        }
    }

    dom_to_ratatui(&dom.document, Style::default().fg(Color::Red))
}
