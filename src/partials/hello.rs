use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "hello.html")]

struct HelloTemplate {
    name: String,
}

pub async fn to_html() -> impl IntoResponse {
    HelloTemplate {
        name: String::from("world"),
    }
}
