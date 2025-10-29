use quick_xml::{events::Event, reader::Reader};
use ratatui::Text;
use std::error::Error;

pub fn xml_to_ratatui(xml: &[u8]) -> Result<Text, Box<dyn Error>> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);

    let mut text = Text::default();
    loop {
        text.push_line(reader.read_event()?)
    }
    Ok(Text("text"))
}
