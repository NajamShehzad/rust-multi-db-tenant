// src/user/user_handler.rs
use actix_web::{web, HttpResponse, HttpRequest};
use std::sync::Arc;
use crate::utils::{extract_db_name, log_memory_usage};

use crate::user::user_model::User;
use crate::user::user_service;
use crate::db::MongoRepo;
use crate::user::user_errors::UserServiceError;

pub async fn create_user(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
    user: web::Json<User>,
) -> HttpResponse {
    let db_name = extract_db_name(&req);
    let user_data = user.into_inner();
    match user_service::create_user_service(&db_name, data.get_ref().clone(), user_data).await {
        Ok(user_id) => HttpResponse::Ok().json(user_id),
        Err(e) => map_service_error_to_response(e),
    }
}

pub async fn get_user(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
    user_id: web::Path<String>,
) -> HttpResponse {
    let db_name = extract_db_name(&req);

    match user_service::get_user_service(&db_name, data.get_ref().clone(), &user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => map_service_error_to_response(e),
    }
}

pub async fn update_user(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
    user_id: web::Path<String>,
    user: web::Json<User>,
) -> HttpResponse {
    let db_name = extract_db_name(&req);
    let user_data = user.into_inner();

    match user_service::update_user_service(&db_name, data.get_ref().clone(), &user_id, user_data).await {
        Ok(_) => HttpResponse::Ok().body("User updated successfully"),
        Err(e) => map_service_error_to_response(e),
    }
}

pub async fn delete_user(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
    user_id: web::Path<String>,
) -> HttpResponse {
    let db_name = extract_db_name(&req);

    match user_service::delete_user_service(&db_name, data.get_ref().clone(), &user_id).await {
        Ok(_) => HttpResponse::Ok().body("User deleted successfully"),
        Err(e) => map_service_error_to_response(e),
    }
}

pub async fn get_all_users(
    req: HttpRequest,
    data: web::Data<Arc<MongoRepo>>, // Use Arc here
) -> HttpResponse {
    let db_name = extract_db_name(&req);
    log_memory_usage(); // Log memory usage

    match user_service::get_all_users_service(&db_name, data.get_ref().clone()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => map_service_error_to_response(e),
    }
}

fn map_service_error_to_response(error: UserServiceError) -> HttpResponse {
    match error {
        UserServiceError::InvalidId => HttpResponse::BadRequest().body(error.to_string()),
        UserServiceError::NotFound => HttpResponse::NotFound().body(error.to_string()),
        UserServiceError::InsertionFailed => HttpResponse::InternalServerError().body(error.to_string()),
        UserServiceError::DatabaseError(_) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
