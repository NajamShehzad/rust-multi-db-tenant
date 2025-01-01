// src/todo/todo_service.rs
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::FindOptions;
use futures::stream::StreamExt;
use std::sync::Arc;

use crate::todo::todo_model::Todo;
use crate::db::MongoRepo;
use crate::todo::todo_errors::TodoServiceError;

pub async fn create_todo_service(
    db_name: &str,
    data: Arc<MongoRepo>,
    todo: Todo,
) -> Result<ObjectId, TodoServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<Todo>("todos");

    let new_todo = Todo {
        id: None,
        title: todo.title,
        description: todo.description,
        completed: todo.completed,
        user_id: todo.user_id,
    };

    match collection.insert_one(new_todo, None).await {
        Ok(insert_result) => {
            if let Some(id) = insert_result.inserted_id.as_object_id() {
                Ok(id)
            } else {
                Err(TodoServiceError::InsertionFailed)
            }
        }
        Err(e) => Err(TodoServiceError::DatabaseError(e.to_string())),
    }
}

pub async fn get_todo_service(
    db_name: &str,
    data: Arc<MongoRepo>,
    todo_id: &str,
) -> Result<Todo, TodoServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<Todo>("todos");

    let id = ObjectId::parse_str(todo_id).map_err(|_| TodoServiceError::InvalidId)?;

    collection
        .find_one(doc! {"_id": id}, None)
        .await
        .map_err(|e| TodoServiceError::DatabaseError(e.to_string()))?
        .ok_or(TodoServiceError::NotFound)
}

pub async fn update_todo_service(
    db_name: &str,
    data: Arc<MongoRepo>,
    todo_id: &str,
    todo: Todo,
) -> Result<(), TodoServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<Todo>("todos");

    let id = ObjectId::parse_str(todo_id).map_err(|_| TodoServiceError::InvalidId)?;

    let filter = doc! { "_id": id };
    let update = doc! {
        "$set": {
            "title": &todo.title,
            "description": &todo.description,
            "completed": todo.completed,
            "user_id": todo.user_id,
        }
    };

    let update_result = collection.update_one(filter, update, None).await
        .map_err(|e| TodoServiceError::DatabaseError(e.to_string()))?;

    if update_result.matched_count == 1 {
        Ok(())
    } else {
        Err(TodoServiceError::NotFound)
    }
}

pub async fn delete_todo_service(
    db_name: &str,
    data: Arc<MongoRepo>,
    todo_id: &str,
) -> Result<(), TodoServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<Todo>("todos");

    let id = ObjectId::parse_str(todo_id).map_err(|_| TodoServiceError::InvalidId)?;

    let filter = doc! { "_id": id };

    let delete_result = collection.delete_one(filter, None).await
        .map_err(|e| TodoServiceError::DatabaseError(e.to_string()))?;

    if delete_result.deleted_count == 1 {
        Ok(())
    } else {
        Err(TodoServiceError::NotFound)
    }
}

pub async fn get_all_todos_service(
    db_name: &str,
    data: Arc<MongoRepo>,
) -> Result<Vec<Todo>, TodoServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<Todo>("todos");

    let find_options = FindOptions::builder().build();

    let mut cursor = collection.find(None, find_options).await
        .map_err(|e| TodoServiceError::DatabaseError(e.to_string()))?;

    let mut todos = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(todo) => todos.push(todo),
            Err(e) => return Err(TodoServiceError::DatabaseError(e.to_string())),
        }
    }
    Ok(todos)
}
