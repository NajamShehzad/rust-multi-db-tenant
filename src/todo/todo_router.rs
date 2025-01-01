// src/todo/todo_router.rs
use actix_web::web;
use crate::todo::todo_handler::*;

pub fn init_todo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("", web::post().to(create_todo))
            .route("", web::get().to(get_all_todos))
            .route("/{id}", web::get().to(get_todo))
            .route("/{id}", web::put().to(update_todo))
            .route("/{id}", web::delete().to(delete_todo)),
    );
}
