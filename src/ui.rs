use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
};
use std::error::Error;

use crate::{
    app::{App, Screen},
    styling::html_to_ratatui,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Max(app.text_width), Constraint::Min(1)])
        .split(frame.area());

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

            frame.render_stateful_widget(feed_list, chunks[0], &mut app.index.state);

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
                    frame.render_widget(posts_list, chunks[1]);
                }
                None => {
                    frame.render_widget(Paragraph::new("No feed selected."), chunks[1]);
                }
            }
        }
        Screen::FeedMenu => {
            let mut feed_titles = Vec::<ListItem>::new();

            app.index.meta.iter().for_each(|meta| {
                feed_titles.push(ListItem::new(Line::from(Span::styled(
                    format!("{}", meta.title),
                    Style::default().fg(Color::Yellow),
                ))));
            });

            let feed_list = List::new(feed_titles)
                .highlight_style(Style::new().bg(Color::Green).add_modifier(Modifier::BOLD));

            frame.render_stateful_widget(feed_list, chunks[0], &mut app.index.state);

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
        Screen::Reader => {
            match Reader::new(app) {
                Ok(mut r) => {
                    frame.render_widget(r.paragraph, chunks[1]);
                    frame.render_stateful_widget(r.scrollbar, chunks[1], &mut r.scrollbar_state);
                }
                Err(e) => println!("error: {}", e),
            };
        }
        _ => {}
    }
}

pub struct Reader<'a> {
    paragraph: Paragraph<'a>,
    scrollbar: Scrollbar<'a>,
    scrollbar_state: ScrollbarState,
}

impl Reader<'_> {
    fn new(app: &App) -> Result<Reader<'_>, Box<dyn Error>> {
        let ch = app.index.state.selected().unwrap();
        let p = app.feeds[ch].state.selected().unwrap();

        let text = html_to_ratatui(app.feeds[ch].posts[p].content.as_bytes());
        let title = app.feeds[ch].posts[p].title.as_str();
        let mut scroll = app.feeds[ch].posts[p].scroll;

        let paragraph = Paragraph::new(text)
            .style(Style::new().fg(Color::DarkGray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Line::from(title))
                    .border_type(BorderType::Rounded)
                    .padding(Padding::symmetric(8, 0)),
            )
            .wrap(Wrap { trim: false }).scroll(scroll);

        scroll.0 = scroll.0.clamp(0, paragraph.line_count(app.text_width) as u16);

        let scrollbar_state =
            ScrollbarState::new(paragraph.line_count(app.text_width)).position(scroll.0.into());

        Ok(Reader {
            paragraph: paragraph,
            scrollbar: Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("^"))
                .end_symbol(Some("v")),
            scrollbar_state: scrollbar_state,
        })
    }
}
