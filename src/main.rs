use std::{error::Error, io};

use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

mod app;
mod ui;

use crate::{
    app::{App, Screen},
    ui::ui,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    app.add_channel("https://ictnews.org/feed").await?;
    app.add_channel("https://daniel.haxx.se/blog/feed/").await?;

    app.load_all()?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {}
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                Screen::MainMenu => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = Screen::Exiting;
                    }
                    KeyCode::Char('j') => app.index.state.select_next(),
                    KeyCode::Char('k') => app.index.state.select_previous(),
                    KeyCode::Enter => {
                        app.current_screen = Screen::FeedMenu;
                    }
                    _ => {}
                },
                Screen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                Screen::Reader => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = Screen::FeedMenu;
                    }
                    KeyCode::Char('j') => {
                        let ch = app.index.state.selected().unwrap();
                        let p = app.feeds[ch].state.selected().unwrap();
                        app.feeds[ch].posts[p].scroll_down();
                    }
                    KeyCode::Char('k') => {
                        let ch = app.index.state.selected().unwrap();
                        let p = app.feeds[ch].state.selected().unwrap();
                        app.feeds[ch].posts[p].scroll_up();
                    }
                    _ => {}
                }
                Screen::FeedMenu => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = Screen::MainMenu;
                    }
                    KeyCode::Char('j') => {
                        let ch = app.index.state.selected().unwrap();
                        app.feeds[ch].state.select_next();
                    }
                    KeyCode::Char('k') => {
                        let ch = app.index.state.selected().unwrap();
                        app.feeds[ch].state.select_previous();
                    }
                    KeyCode::Enter => {
                        let ch = app.index.state.selected().unwrap();
                        let p = app.feeds[ch].state.selected().unwrap();
                        app.current_screen = Screen::Reader;
                    }
                    _ => {}
                },
            }
        }
    }
}
