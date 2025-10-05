use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, Screen};

pub fn ui(frame: &mut Frame, app: &mut App) {
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

    match &app.current_screen {
        Screen::MainMenu => {
            let mut feed_titles = Vec::<ListItem>::new();

            for feed in &app.channels.items {
                feed_titles.push(ListItem::new(Line::from(Span::styled(
                    format!("{}", feed.channel.title()),
                    Style::default().fg(Color::Yellow),
                ))));
            }

            let feed_list = List::new(feed_titles)
                .highlight_style(Style::new().bg(Color::Green).add_modifier(Modifier::BOLD));

            frame.render_stateful_widget(feed_list, chunks[1], &mut app.channels.state);
        }
        Screen::FeedMenu => {
            let mut post_titles = Vec::<ListItem>::new();

            match app.channels.currently_viewing {
                Some(i) => {
                    for post in app.channels.items[i].channel.items() {
                        post_titles.push(ListItem::new(Line::from(Span::styled(
                            format!("{}", post.title().unwrap()),
                            Style::default().fg(Color::Green),
                        ))));
                    }
                    let posts_list = List::new(post_titles).highlight_style(
                        Style::new().bg(Color::Green).add_modifier(Modifier::BOLD),
                    );
                    frame.render_stateful_widget(posts_list, chunks[1], &mut app.channels.items[i].state);
                }
                None => {
                    frame.render_widget(Paragraph::new("No feed selected."), chunks[1]);
                }
            }
        }
        _ => {}
    }
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
