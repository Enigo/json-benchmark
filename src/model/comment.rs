use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Comments {
    pub comments: Vec<Comment>,
}

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub body: String,
    #[serde(rename = "postId")]
    pub post_id: u32,
    pub user: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
}
