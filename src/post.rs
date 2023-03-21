use gloo_net::http::Request;
use serde::Deserialize;
use yew::{html, Html};

#[derive(Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    user_id: usize,
    pub id: usize,
    pub title: String,
    pub body: String,
}

pub async fn fetch_posts() -> Vec<Post> {
    let fetched_posts: Vec<Post> = Request::get("https://jsonplaceholder.typicode.com/posts")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    fetched_posts
}

pub fn render_post(post: &Post) -> Html {
    html! {
        <div style="display: flex; flex-direction: column; padding: 10px; border: 1px solid black; border-radius: 5px; width: 300px; margin: 10px;">
            <div style="font-size: 1.3em; padding-bottom: 10px; border-bottom: 1px solid black; font-weight: 500;">{format!("{}", post.title)}</div>
            <div>{format!("{}", post.body)}</div>
        </div>
    }
}
