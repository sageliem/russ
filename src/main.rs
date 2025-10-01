use std::error::Error;
use std::io;
use std::vec::Vec;
use rss::Channel;
use rss::validation::Validate;
use crossterm::event::{self, Event};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame};

#[derive(Debug, Default)]
pub struct App {
    feeds: Vec<Channel>,
    exit: bool,
}

async fn get_channel(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let channel = get_channel("https://100r.ca/links/rss.xml")
    let channel = get_channel("https://ictnews.org/feed/")
        .await?;
    channel.validate().unwrap();

    let mut terminal = ratatui::init();
    let mut app = App::default();
    app.feeds.push(channel);

    let result = app.run(&mut terminal);
    ratatui::restore();
   Ok(()) 
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Reader".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let text: &str = &self.feeds[0].items()[0].content().unwrap();
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl App {

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if matches!(event::read()?, Event::Key(_)) {
            self.exit = true;
        }
        Ok(())
    }
}
