// use quick_xml::{events::Event, reader::Reader};
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

pub fn html_to_ratatui(mut html: &[u8]) -> Text {
    let dom = html5ever::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html)
        .unwrap();

    fn dom_to_ratatui(node: &Handle, parent_style: Style) -> Text<'static> {
        match &node.data {
            NodeData::Text { contents } => {
                let s = contents.borrow().to_string();
                Text::from(Span::styled(s, parent_style))
            }
            NodeData::Element { name, .. } => {
                let mut lines: Vec<Line> = Vec::new();
                let mut style = parent_style;
                let tag_name = name.local.as_ref();
                match tag_name {
                    "h1" => {
                        style = style
                            .add_modifier(Modifier::BOLD)
                            .add_modifier(Modifier::UNDERLINED)
                    }
                    "b" | "strong" => {
                        style = style.add_modifier(Modifier::BOLD);
                    }
                    "em" => {
                        style = style.add_modifier(Modifier::ITALIC);
                    }
                    "a" => style = style.add_modifier(Modifier::UNDERLINED).fg(Color::Blue),
                    _ => {}
                }
                for child in &node.children.clone().into_inner() {
                    let child_text = dom_to_ratatui(child, style);
                    lines.extend(child_text.lines.into_iter().clone().collect::<Vec<_>>());
                }
                Text::from(lines)
            }
            NodeData::Document => {
                let mut lines: Vec<Line> = Vec::new();
                for child in &node.children.clone().into_inner() {
                    let child_text = dom_to_ratatui(child, parent_style);
                    lines.extend(child_text.lines.into_iter().clone().collect::<Vec<_>>());
                }
                Text::from(lines)
            }
            _ => Text::raw("blah"),
        }
    }

    dom_to_ratatui(&dom.document, Style::default().fg(Color::Red))
}
