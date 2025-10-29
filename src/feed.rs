use atom_syndication::{Entry, Feed};
use rss::{Channel, Item};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    env,
    error::Error,
    fs::{self, File},
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
};

// TODO remove
use html2text::from_read;
use ratatui::{crossterm::terminal, widgets::ListState};

use crate::styling;

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub id: String,
    pub title: String,
    pub url: String,
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

    pub fn from_atom(item: &Entry) -> Result<Post, Box<dyn Error>> {
        let title: String = item.title().to_string();

        let html_content = match item.content() {
            Some(t) => match t.value() {
                Some(t) => t,
                None => "Could not get value of content",
            }
            .as_bytes(),
            None => "Could not get content from post.".as_bytes(),
        };
        let text_content = match from_read(html_content, usize::from(terminal::size().unwrap().0)) {
            Ok(t) => t,
            Err(e) => e.to_string(),
        };

        Ok(Post {
            title: if title.is_empty() {
                "[untitled]".to_string()
            } else {
                title
            },
            content: text_content,
            scroll: (0, 0),
        })
    }

    pub fn scroll_up(&mut self) {
        if self.scroll.0 > 0 {
            self.scroll.0 -= 1
        };
    }
    pub fn scroll_down(&mut self) {
        self.scroll.0 += 1
    }
}

#[derive(Serialize, Deserialize)]
pub struct RussFeed {
    pub meta: Metadata,
    pub posts: Vec<Post>,
    pub state: ListState,
}

impl RussFeed {
    pub fn from_rss(channel: Channel) -> Result<RussFeed, Box<dyn Error>> {
        let mut posts = Vec::new();
        for item in channel.items() {
            posts.push(Post::from_item(item)?);
        }

        let mut hasher = DefaultHasher::new();
        channel.link().hash(&mut hasher);
        Ok(RussFeed {
            meta: Metadata {
                id: hasher.finish().to_string(),
                title: channel.title().to_string(),
                url: channel.link().to_string(),
            },
            posts: posts,
            state: ListState::default().with_selected(Some(0)),
        })
    }

    pub fn from_atom(atom_feed: Feed) -> Result<RussFeed, Box<dyn Error>> {
        let mut posts = Vec::new();
        for item in atom_feed.entries() {
            posts.push(Post::from_atom(item)?)
        }

        let mut hasher = DefaultHasher::new();
        atom_feed.id().hash(&mut hasher);

        Ok(RussFeed {
            meta: Metadata {
                id: hasher.finish().to_string(),
                title: atom_feed.title().to_string(),
                url: atom_feed.id().to_string(),
            },
            posts: posts,
            state: ListState::default().with_selected(Some(0)),
        })
    }

    pub fn from_file(feed_id: String) -> Result<RussFeed, Box<dyn Error>> {
        let mut path = env::home_dir().unwrap();
        path.push(".russ/feeds/");
        path.push(feed_id);
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&self).unwrap();

        let mut hasher = DefaultHasher::new();
        self.meta.url.hash(&mut hasher);
        let mut path = env::home_dir().unwrap();
        path.push(".russ/feeds/");
        path.push(hasher.finish().to_string());
        if !path.exists() {
            let mut file = File::create(path)?;
            file.write_all(serialized.as_ref())?;
        }

        Ok(())
    }
}
