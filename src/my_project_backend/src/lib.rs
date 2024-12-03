use std::cell::RefCell;

use crate::blog::Blog;

mod blog;

thread_local! {
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog>{
    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow().clone())
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<String, String> {

    if title.len() > 255 {
        return Err("Blog title is too long!".to_string())
    }

    let new_blog: Blog = Blog::new(title, content, tags);
    BLOGS.with(|blogs: &RefCell<Vec<Blog>>| blogs.borrow_mut().push(new_blog));

    Ok("Added new blog".to_string())
}
