use axum::{
    extract::{State},
    http::StatusCode,
    routing::{post},
    Json,
    Router,
};

use serde::{Deserialize,Serialize};
use std::sync::{Arc,Mutex};



#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser{
    name: String,
    email: String,
}

type AppState = Arc<Mutex<Vec<User>>>;

async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUser>,
)-> (StatusCode, Json<User>){
    let mut users = state.lock().unwrap();
    let new_id = users.last().map_or(1, |u| u.id + 1);
    let new_user = User {
        id: new_id,
        name: body.name,
        email: body.email,
    };
    users.push(new_user.clone());
    (StatusCode::CREATED, Json(new_user))
}


#[tokio::main]
async fn main(){
    let state: AppState = Arc::new(Mutex::new(vec![]));

    let app = Router::new()
        .route("/users",post(create_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");
    axum::serve(listener,app).await.unwrap();
}

