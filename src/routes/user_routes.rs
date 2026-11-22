use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use crate::error::AppError;
use crate::models::user::{CreateUserRequest, User};
use crate::states::AppState;

// 1. Get all users
async fn get_all_users(State(state): State<AppState>) -> (StatusCode, Json<Vec<User>>) {
    let users = state.users.lock().unwrap();
    (StatusCode::OK, Json(users.clone()))
}

// 2. Get a single user by ID: /users/{id}
async fn get_user_by_id(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<User>, AppError> {
    let users = state.users.lock().unwrap();
    users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or_else(|| AppError::NotFound(format!("User with ID {} was not found", id)))
}

// 3. POST a new user: /users
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    if payload.name.trim().is_empty() {
        return Err(AppError::BadRequest("User name cannot be empty".to_string()));
    }
    if payload.email.trim().is_empty() {
        return Err(AppError::BadRequest("User email cannot be empty".to_string()));
    }

    let mut users = state.users.lock().unwrap();

    // Prevent duplicate emails
    if users.iter().any(|u| u.email == payload.email) {
        return Err(AppError::BadRequest(format!(
            "User with email '{}' already exists",
            payload.email
        )));
    }

    // Auto-increment ID based on the last user ID
    let new_id = users.last().map_or(1, |u| u.id + 1);
    let new_user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };
    users.push(new_user.clone());

    Ok((StatusCode::CREATED, Json(new_user)))
}

// Export router carrying AppState
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_users))
        .route("/{id}", get(get_user_by_id))
        .route("/", post(create_user))
}