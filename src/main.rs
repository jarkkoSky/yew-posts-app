use crate::post::{fetch_posts, render_post};
use yew::prelude::*;

pub mod post;

#[function_component]
fn App() -> Html {
    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    posts.set(fetch_posts().await);
                });
            },
            (),
        );
    }

    html! {
        <div style="display: flex; flex-wrap: wrap;">
            {posts.iter().map(render_post).collect::<Html>()}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
