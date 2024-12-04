use std::cell::RefCell;

use crate::blog::Blog;
use crate::config::Config;

mod blog;
mod config;

thread_local! {
    static CONFIG: RefCell<Config> = RefCell::new(Config::new());
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog>{
    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow().clone())
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<Blog, String> {

    ic_cdk::println!("New blog: (title: {}, content: ..., tags: {:?}", title, tags);

    let config = CONFIG.with(|config| config.borrow().clone());

    if title.len() > config.max_title_length as usize {
        return Err("Blog title is too long!".to_string())
    }
    if content.len() > config.max_content_length as usize {
        return Err("Blog content is too long!".to_string())
    }
    if tags.len() > config.max_tags as usize {
        return Err("Blog has too many tags!".to_string())
    }

    let tags_valid: bool = tags.iter().any(|tag| !config.tags.contains(tag));
    if tags_valid {
        return Err("Tags are invalid".to_string())
    }
    
    let new_blog: Blog = Blog::new(title, content, tags);
    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| 
        blogs.borrow_mut()
        .push(new_blog)
    );

    let last_blog: Blog = BLOGS.with(|blogs: &RefCell<Vec<Blog>>| 
        blogs.borrow()
        .last()
        .expect("Blog list is empty!")
        .clone()
    );

    Ok(last_blog)
}

#[ic_cdk::update]
fn add_config(new_config: Config) -> () {
    CONFIG.with(|config| *config.borrow_mut() = new_config);
}
