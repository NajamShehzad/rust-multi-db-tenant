// src/routes/mod.rs
use actix_web::web;
use crate::user::user_router::init_user_routes;
use crate::todo::todo_router::init_todo_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(init_user_routes);
    cfg.configure(init_todo_routes);
}
