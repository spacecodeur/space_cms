use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    Latest,
    Popular,
}

#[derive(Clone, Debug)]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub teaser_image: String,
    pub author_avatar: String,
    pub author_name: String,
    pub date: DateTime<Utc>,
    pub views: u32,
}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id 
            && self.title == other.title 
            && self.teaser_image == other.teaser_image
            && self.author_avatar == other.author_avatar
            && self.author_name == other.author_name
            && self.date.timestamp() == other.date.timestamp()
            && self.views == other.views
    }
}