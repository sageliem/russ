use atom_syndication::Feed;
use quick_xml::{self, events};
use ratatui::widgets::ListState;
use rss::Channel;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
};

use crate::feed;

pub enum FeedType {
    RSS,
    Atom,
}

pub enum Screen {
    Reader,
    MainMenu,
    FeedMenu,
    Exiting,
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub meta: Vec<feed::Metadata>,
    pub state: ListState,
}

impl Index {
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut path = env::home_dir().unwrap();
        path.push(".russ/index.json");
        let mut file = File::create(path)?;
        file.write_all(serialized.as_ref())?;

        Ok(())
    }

    pub fn from_file() -> Result<Index, Box<dyn Error>> {
        let mut path = env::home_dir().unwrap();
        path.push(".russ/index.json");
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub feed_dir: String,
    pub config_dir: String,
    pub feeds: Vec<String>,
}

pub struct App {
    pub current_screen: Screen,
    pub index: Index,
    pub feeds: Vec<feed::RussFeed>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: Screen::MainMenu,
            index: Index {
                meta: Vec::new(),
                state: ListState::default().with_selected(Some(0)),
            },
            feeds: Vec::new(),
        }
    }

    pub async fn load_from_config(&mut self) -> Result<(), Box<dyn Error>> {
        let mut path = env::home_dir().unwrap();
        path.push(".config/russ/");
        path.push("config.toml");
        let config_str = fs::read_to_string(path).expect("Failed to read configuration file.");
        let config: Config = toml::from_str(&config_str)?;

        for url in config.feeds {
            println!("Adding feed {}", url);
            self.add_channel(&url.as_str()).await?;
        }

        Ok(())
    }

    pub fn save_index(&self) {
        self.index.save();
    }

    pub fn load_all(&mut self) -> Result<(), Box<dyn Error>> {
        let mut path = env::home_dir().unwrap();
        path.push(".russ/feeds/");
        _ = fs::create_dir(path);

        self.index = Index::from_file()?;

        self.index.meta.iter().for_each(|meta| {
            self.feeds
                .push(feed::RussFeed::from_file(meta.id.clone()).unwrap());
        });

        Ok(())
    }

    pub async fn add_channel(&mut self, url: &str) -> Result<(), Box<dyn Error>> {
        // check if channel already exists
        // for m in &self.index.meta {
        //     if m.url == url.to_string() {
        //         return Ok(());
        //     }
        // }
        let mut path = env::home_dir().unwrap();
        path.push(".russ/feeds/");

        if !path.exists() {
            fs::create_dir(path).expect("Failed to create feed dir");
        }

        let content = reqwest::get(url).await?.bytes().await?;
        let xml = content.as_ref();

        let mut feed_type: Option<FeedType> = None;
        // Check content type
        let mut reader = quick_xml::Reader::from_str(std::str::from_utf8(xml)?);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                // exits the loop when reaching end of file
                Ok(events::Event::Eof) => break,
                Ok(events::Event::Start(e)) => match e.name().as_ref() {
                    b"rss" => feed_type = Some(FeedType::RSS),
                    b"feed" => feed_type = Some(FeedType::Atom),
                    _ => (),
                },
                _ => (),
            }
        }

        let feed: feed::RussFeed = match feed_type {
            Some(FeedType::RSS) => feed::RussFeed::from_rss(Channel::read_from(&xml[..])?)?,
            Some(FeedType::Atom) => feed::RussFeed::from_atom(Feed::read_from(&xml[..])?)?,
            None => return Err("Invalid feed syntax".into()),
        };

        self.index.meta.push(feed.meta.clone());
        _ = feed.save();
        self.save_index();
        Ok(())
    }
}
