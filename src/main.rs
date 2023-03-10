mod model;

use crate::model::comment::Comments;

fn main() {
    execute_request();
}

#[tokio::main]
async fn execute_request() {
    match reqwest::get("https://dummyjson.com/comments").await.unwrap().json::<Comments>().await {
        Ok(comment) => {
            println!("{:?}", comment.comments.len())
        }
        Err(e) => {
            eprint!("{e}")
        }
    }
}
