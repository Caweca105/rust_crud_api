use serde::{Deserialize, Serialize};
use uuid::Uuid;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

// Struct for incoming user data (no id)
#[derive(Serialize, Deserialize)]
struct NewUser {
    name: String,
    age: i32,
}

// Struct for complete user (with id)
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Uuid,
    name: String,
    age: i32,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
            .route("/users", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_users(app_state: web::Data<AppState>) -> impl Responder {
    let users = app_state.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

async fn create_user(app_state: web::Data<AppState>, new_user: web::Json<NewUser>) -> impl Responder {
    let mut users = app_state.users.lock().unwrap();

    // Create a new user with a generated id
    let user = User {
        id: Uuid::new_v4(),
        name: new_user.name.clone(),
        age: new_user.age,
    };

    users.push(user.clone());
    HttpResponse::Created().json(user) // Return the created user with the id
}

async fn update_user(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let mut users = app_state.users.lock().unwrap();
    let index = users.iter().position(|u| u.id == user.id).unwrap();
    users[index] = user.into_inner();
    HttpResponse::Ok().finish()
}

async fn delete_user(app_state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let mut users = app_state.users.lock().unwrap();
    let index = users.iter().position(|u| u.id == *user_id).unwrap();
    users.remove(index);
    HttpResponse::Ok().finish()
}