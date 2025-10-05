use rss::{Channel, Item};
use std::{
    error::Error,
    fs::{self, DirEntry, File},
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufReader, Write},
    path::Path,
};
use ratatui::widgets::ListState;

pub enum Screen {
    Reader,
    MainMenu,
    FeedMenu,
    Exiting,
}

impl Default for Screen {
    fn default() -> Screen {
        Screen::MainMenu
    }
}

pub struct ChannelList {
    pub items: Vec<Feed>,
    pub state: ListState,
    pub currently_viewing: Option<usize>,
}

pub struct Feed {
    pub channel: Channel,
    pub state: ListState,
    pub currently_viewing: Option<usize>,
}

impl Feed {
    pub fn from(channel: Channel) -> Feed {
        Feed {
            channel: channel,
            state: ListState::default().with_selected(Some(0)),
            currently_viewing: None,
        }
    }
}

pub struct App {
    pub current_screen: Screen,
    pub channels: ChannelList,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: Screen::default(),
            channels: ChannelList {
                items: Vec::new(),
                state: ListState::default().with_selected(Some(0)),
                currently_viewing: None,
            },
        }
    }

    fn make_feed_dir() -> Result<(), Box<dyn Error>> {
        fs::create_dir("feeds")?;
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        _ = App::make_feed_dir();

        for feed in fs::read_dir(&Path::new("feeds/"))? {
            let feed = File::open(feed?.path())?;
            let channel = Channel::read_from(BufReader::new(feed))?;
            self.channels.items.push(Feed::from(channel));
        }

        Ok(())
    }

    pub async fn add_channel(&mut self, url: &str) -> Result<(), Box<dyn Error>> {
        _ = App::make_feed_dir();

        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let path = Path::new("feeds/").join(hasher.finish().to_string());
        if !path.exists() {
            let content = reqwest::get(url).await?.bytes().await?;
            let mut file = File::create(path)?;
            file.write_all(content.as_ref())?;
        }
        Ok(())
    }
}
