use axum::{
    Json,
    extract::{Path, State, rejection::JsonRejection},
    http::StatusCode,
};
use tracing::instrument;

use crate::{
    error::AppError,
    models::{CreateTodo, Todo, UpdateTodo},
    state::AppState,
    validation::validate,
};

fn parse_json<T>(result: Result<Json<T>, JsonRejection>) -> Result<T, AppError> {
    result
        .map(|Json(value)| value)
        .map_err(|_| AppError::BadRequest)
}

#[instrument(skip(state, payload))]
pub async fn create_todo(
    State(state): State<AppState>,
    payload: Result<Json<CreateTodo>, JsonRejection>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let mut payload = validate(parse_json(payload)?)?;
    payload.title = payload.title.trim().to_owned();

    let id = state.allocate_id().ok_or(AppError::IdExhausted)?;

    let todo = Todo {
        id,
        title: payload.title,
        description: payload.description,
        completed: false,
    };

    state.todos.write().await.insert(id, todo.clone());

    Ok((StatusCode::CREATED, Json(todo)))
}

#[instrument(skip(state))]
pub async fn get_todos(State(state): State<AppState>) -> (StatusCode, Json<Vec<Todo>>) {
    let todos = state.todos.read().await;

    let mut results: Vec<Todo> = todos.values().cloned().collect();
    results.sort_unstable_by_key(|todo| todo.id);

    (StatusCode::OK, Json(results))
}

#[instrument(skip(state))]
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, AppError> {
    let todos = state.todos.read().await;

    todos.get(&id).cloned().map(Json).ok_or(AppError::NotFound)
}

#[instrument(skip(state, payload))]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    payload: Result<Json<UpdateTodo>, JsonRejection>,
) -> Result<Json<Todo>, AppError> {
    let mut payload = validate(parse_json(payload)?)?;
    payload.title = payload.title.trim().to_owned();

    let mut todos = state.todos.write().await;
    let todo = todos.get_mut(&id).ok_or(AppError::NotFound)?;

    todo.title = payload.title;
    todo.description = payload.description;
    todo.completed = payload.completed;

    Ok(Json(todo.clone()))
}

#[instrument(skip(state))]
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, AppError> {
    let mut todos = state.todos.write().await;

    todos
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(AppError::NotFound)
}
