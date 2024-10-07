use axum::{
    routing::post,
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(receive_word));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn receive_word(Json(payload): Json<WordEvent>, ) ->  (StatusCode, Json<WordEvent>) {
    let word_event = WordEvent {
        title: payload.title,
        word: payload.word,
        word_number: payload.word_number
    };

    println!("{:?}", word_event);

    (StatusCode::CREATED, Json(word_event))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WordEvent {
    pub title: String,
    pub word: String,
    pub word_number: u64 
}