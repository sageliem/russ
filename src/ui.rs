use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use std::error::Error;

use crate::{
    app::{App, Screen},
    styling::html_to_ratatui,
};

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

            app.index.meta.iter().for_each(|meta| {
                feed_titles.push(ListItem::new(Line::from(Span::styled(
                    format!("{}", meta.title),
                    Style::default().fg(Color::Yellow),
                ))));
            });

            let feed_list = List::new(feed_titles)
                .highlight_style(Style::new().bg(Color::Green).add_modifier(Modifier::BOLD));

            frame.render_stateful_widget(feed_list, chunks[1], &mut app.index.state);
        }
        Screen::FeedMenu => {
            let mut post_titles = Vec::<ListItem>::new();

            match app.index.state.selected() {
                Some(i) => {
                    for post in &app.feeds[i].posts {
                        post_titles.push(ListItem::new(Line::from(Span::styled(
                            format!("{}", post.title),
                            Style::default().fg(Color::Green),
                        ))));
                    }
                    let posts_list = List::new(post_titles)
                        .highlight_style(Style::new().bg(Color::Red).add_modifier(Modifier::BOLD));
                    frame.render_stateful_widget(posts_list, chunks[1], &mut app.feeds[i].state);
                }
                None => {
                    frame.render_widget(Paragraph::new("No feed selected."), chunks[1]);
                }
            }
        }
        Screen::Reader => match reader(app) {
            Ok(paragraph) => frame.render_widget(paragraph, chunks[1]),
            Err(e) => println!("error: {}", e),
        },
        _ => {}
    }
}

fn reader(app: &App) -> Result<Paragraph, Box<dyn Error>> {
    let ch = app.index.state.selected().unwrap();
    let p = app.feeds[ch].state.selected().unwrap();
    // let text = app.feeds[ch].posts[p].content.clone();
    let text = html_to_ratatui(app.feeds[ch].posts[p].content.as_bytes());
    Ok(Paragraph::new(text)
        .style(Style::new().fg(Color::DarkGray))
        .wrap(Wrap { trim: true })
        .scroll(app.feeds[ch].posts[p].scroll))
}
