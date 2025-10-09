use html2text::from_read;
use ratatui::{crossterm::terminal, widgets::ListState};
use rss::{Channel, Item};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use std::{
    error::Error,
    fs::{self, DirEntry, File},
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufReader, Write},
    path::Path,
};
use toml::Table;

pub enum Screen {
    Reader,
    MainMenu,
    FeedMenu,
    Exiting,
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub meta: Vec<Metadata>,
    pub state: ListState,
}

impl Index {
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&self).unwrap();
        let path = Path::new("feeds/index.json");
        let mut file = File::create(path)?;
        file.write_all(serialized.as_ref())?;

        Ok(())
    }

    pub fn from_file() -> Result<Index, Box<dyn Error>> {
        let path = Path::new("feeds/index.json");
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Feed {
    pub meta: Metadata,
    pub posts: Vec<Post>,
    pub state: ListState,
}

#[derive(Deserialize)]
pub struct Config {
    pub feed_dir: String,
    pub config_dir: String,
    pub feeds: Vec<String>,
}

impl Feed {
    pub fn from_channel(channel: Channel) -> Result<Feed, Box<dyn Error>> {
        let mut posts = Vec::new();
        for item in channel.items() {
            posts.push(Post::from_item(item)?);
        }

        let mut hasher = DefaultHasher::new();
        channel.link().hash(&mut hasher);
        Ok(Feed {
            meta: Metadata {
                id: hasher.finish().to_string(),
                title: channel.title().to_string(),
                url: channel.link().to_string(),
            },
            posts: posts,
            state: ListState::default().with_selected(Some(0)),
        })
    }

    pub fn from_file(feed_id: String) -> Result<Feed, Box<dyn Error>> {
        let path = Path::new("feeds/").join(feed_id);
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&self).unwrap();

        let mut hasher = DefaultHasher::new();
        self.meta.url.hash(&mut hasher);
        let path = Path::new("./feeds/").join(hasher.finish().to_string());
        if !path.exists() {
            let mut file = File::create(path)?;
            file.write_all(serialized.as_ref())?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub scroll: (u16, u16),
}

impl Post {
    pub fn from_item(item: &Item) -> Result<Post, Box<dyn Error>> {
        let title = match item.title() {
            Some(t) => t.to_string(),
            None => "title not found".to_string(),
        };
        let html_content = match item.content() {
            Some(t) => t.as_bytes(),
            None => "Could not get content from post.".as_bytes(),
        };
        let text_content = match from_read(html_content, usize::from(terminal::size().unwrap().0)) {
            Ok(t) => t,
            Err(e) => e.to_string(),
        };

        Ok(Post {
            title: title,
            content: text_content,
            scroll: (0, 0),
        })
    }
    pub fn scroll_up(&mut self) {
        if (self.scroll.0 > 0) {
            self.scroll.0 -= 1
        };
    }
    pub fn scroll_down(&mut self) {
        self.scroll.0 += 1
    }
}

pub struct App {
    pub current_screen: Screen,
    pub index: Index,
    pub feeds: Vec<Feed>,
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
        let path = Path::new("./config.toml");
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
        _ = fs::create_dir("feeds");
        self.index = Index::from_file()?;

        self.index.meta.iter().for_each(|meta| {
            self.feeds.push(Feed::from_file(meta.id.clone()).unwrap());
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
        if !Path::exists(Path::new("feeds")) {
            fs::create_dir("./feeds/").expect("Failed to create feed dir");
        }

        let content = reqwest::get(url).await?.bytes().await?;
        match Channel::read_from(&content[..]) {
            Ok(channel) => {
                let feed = Feed::from_channel(channel)?;
                self.index.meta.push(feed.meta.clone());
                _ = feed.save();
                self.save_index();
            },
            Err(e) => {
                println!("Failed to add channel: {}", e)
            }
        }
        Ok(())
    }
}

fn create_feed_dir() -> Result<(), Box<dyn Error>> {
    _ = fs::create_dir("feeds")?;
    Ok(())
}
