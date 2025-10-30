use quick_xml::{events::Event, reader::Reader};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
};

pub fn xml_to_ratatui(xml: &[u8]) -> Text {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);
    // println!("{}", str::from_utf8(xml).unwrap());

    let mut text = Text::default();
    let mut buf = Vec::new();
    let mut tags: Vec<Vec<u8>> = Vec::new();
    tags.push(b"p".as_ref().to_owned());
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error: {}", e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(ref e)) => tags.push(e.name().as_ref().to_owned()),
            Ok(Event::End(e)) => {
                tags.pop();
                ()
            }
            Ok(Event::Text(e)) => text.push_span(match tags.last().unwrap().as_slice() {
                b"h1" => Span::styled(
                    e.decode().unwrap().into_owned(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                b"strong" => Span::styled(
                    e.decode().unwrap().into_owned(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                b"p" => {
                    let mut t = e.decode().unwrap().into_owned();
                    t.push('\n');
                    Span::styled(t, Style::default())
                }
                b"em" => Span::styled(
                    e.decode().unwrap().into_owned(),
                    Style::default().add_modifier(Modifier::ITALIC),
                ),
                _ => Span::from(e.decode().unwrap().into_owned()),
            }),
            _ => {}
        }
    }
    text
}
