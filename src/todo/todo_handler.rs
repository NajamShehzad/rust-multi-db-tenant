// src/todo/todo_handler.rs
use actix_web::{web, HttpResponse, HttpRequest};
use std::sync::Arc;

use crate::todo::todo_model::Todo;
use crate::todo::todo_service;
use crate::db::MongoRepo;
use crate::todo::todo_errors::TodoServiceError;
use crate::utils::extract_db_name;
use crate::utils::log_memory_usage;

pub async fn create_todo(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
    todo: web::Json<Todo>,
) -> HttpResponse {
    let db_name = extract_db_name(&req);
    let todo_data = todo.into_inner();

    match todo_service::create_todo_service(&db_name, data.get_ref().clone(), todo_data).await {
        Ok(todo_id) => HttpResponse::Ok().json(todo_id),
        Err(e) => map_service_error_to_response(e),
    }
}

pub async fn get_todo(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
    todo_id: web::Path<String>,
) -> HttpResponse {
    let db_name = extract_db_name(&req);

    match todo_service::get_todo_service(&db_name, data.get_ref().clone(), &todo_id).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => map_service_error_to_response(e),
    }
}

pub async fn update_todo(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
    todo_id: web::Path<String>,
    todo: web::Json<Todo>,
) -> HttpResponse {
    let db_name = extract_db_name(&req);
    let todo_data = todo.into_inner();

    match todo_service::update_todo_service(&db_name, data.get_ref().clone(), &todo_id, todo_data).await {
        Ok(_) => HttpResponse::Ok().body("Todo updated successfully"),
        Err(e) => map_service_error_to_response(e),
    }
}

pub async fn delete_todo(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
    todo_id: web::Path<String>,
) -> HttpResponse {
    let db_name = extract_db_name(&req);

    match todo_service::delete_todo_service(&db_name, data.get_ref().clone(), &todo_id).await {
        Ok(_) => HttpResponse::Ok().body("Todo deleted successfully"),
        Err(e) => map_service_error_to_response(e),
    }
}

pub async fn get_all_todos(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
) -> HttpResponse {
    let db_name = extract_db_name(&req);

    log_memory_usage(); // Log memory usage

    match todo_service::get_all_todos_service(&db_name, data.get_ref().clone()).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => map_service_error_to_response(e),
    }
}


fn map_service_error_to_response(error: TodoServiceError) -> HttpResponse {
    match error {
        TodoServiceError::InvalidId => HttpResponse::BadRequest().body(error.to_string()),
        TodoServiceError::NotFound => HttpResponse::NotFound().body(error.to_string()),
        TodoServiceError::InsertionFailed => HttpResponse::InternalServerError().body(error.to_string()),
        TodoServiceError::DatabaseError(_) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
