use std::error::Error;

pub struct Notes {
    title: String,
    content: String,
    author: User,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl Notes {
    pub fn new(title: String, content: String, author: User, created_at: Option<String>, updated_at: Option<String>) -> Self {
        Self { title, content, author, created_at, updated_at }
    }
    
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if Self::is_valid_title(&self.title) {
            return Err("title is empty".into());
        }
        if Self::is_valid_content(&self.content) {
            return Err("content is empty".into());
        }
        if Self::is_valid_author(&self.author) {
            return Err("username is empty".into());
        }
        
        Ok(())
    }
    
    fn is_valid_title(title: &str) -> bool {
        !title.is_empty() && title.len() > 2 
    }
    
    fn is_valid_content(content: &str) -> bool {
        !content.is_empty() && content.len() > 50
    }
    
    fn is_valid_author(author: &User) -> bool {
        !author.username.is_empty()
    }
}

pub struct User {
    username: String,
    last_login: Option<String>,
}

impl User {
    pub fn new(username: String, last_login: Option<String>) -> Self {
        Self { username, last_login }
    }
    
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if !Self::is_valid_username(&self.username) {
            return Err("username is empty".into());
        }
        
        Ok(())
    }
    
    fn is_valid_username(username: &str) -> bool {
        !username.is_empty() && username.len() > 5
    }
}