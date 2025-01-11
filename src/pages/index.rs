use axum::response::{Html, IntoResponse};

pub fn route() -> impl IntoResponse {
    Html("<h1>Hello, World!</h1>")
}
