use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::component::Component;
use crate::app::{App, Screen};

trait Menu {
    fn nav(&mut self, ev: Event) {
        
    }
}

struct ChannelMenu {
    items: Vec<Channel>,
    state: ListState,
}

impl<T: Menu> Component for MainMenu {
    
}

struct PostMenu {
    items: Vec<Item>,
    state: ListState
}

impl<T:Menu> Component for PostMenu {

}

struct Reader {
}



pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("Reader", Style::default().fg(Color::Green)))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut feed_titles = Vec::<ListItem>::new();

    for feed in &app.feeds {
        feed_titles.push(ListItem::new(Line::from(Span::styled(
            format!("{}", feed.title()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let feed_list = List::new(feed_titles);

    frame.render_widget(feed_list, chunks[1]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
