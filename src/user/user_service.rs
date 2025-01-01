// src/user/user_service.rs
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::FindOptions;
use futures::stream::StreamExt;
use std::sync::Arc;

use crate::user::user_model::User;
use crate::db::MongoRepo;
use crate::user::user_errors::UserServiceError;

pub async fn create_user_service(
    db_name: &str,
    data: Arc<MongoRepo>,
    user: User,
) -> Result<ObjectId, UserServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<User>("users");

    let new_user = User {
        id: None,
        name: user.name,
        email: user.email,
        password: user.password,
    };

    match collection.insert_one(new_user, None).await {
        Ok(insert_result) => {
            if let Some(id) = insert_result.inserted_id.as_object_id() {
                Ok(id)
            } else {
                Err(UserServiceError::InsertionFailed)
            }
        }
        Err(e) => Err(UserServiceError::DatabaseError(e.to_string())),
    }
}

pub async fn get_user_service(
    db_name: &str,
    data: Arc<MongoRepo>,
    user_id: &str,
) -> Result<User, UserServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<User>("users");

    let id = ObjectId::parse_str(user_id).map_err(|_| UserServiceError::InvalidId)?;

    collection
        .find_one(doc! {"_id": id}, None)
        .await
        .map_err(|e| UserServiceError::DatabaseError(e.to_string()))?
        .ok_or(UserServiceError::NotFound)
}

pub async fn update_user_service(
    db_name: &str,
    data: Arc<MongoRepo>,
    user_id: &str,
    user: User,
) -> Result<(), UserServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<User>("users");

    let id = ObjectId::parse_str(user_id).map_err(|_| UserServiceError::InvalidId)?;

    let filter = doc! { "_id": id };
    let update = doc! {
        "$set": {
            "name": &user.name,
            "email": &user.email,
            "password": &user.password,
        }
    };

    let update_result = collection.update_one(filter, update, None).await
        .map_err(|e| UserServiceError::DatabaseError(e.to_string()))?;

    if update_result.matched_count == 1 {
        Ok(())
    } else {
        Err(UserServiceError::NotFound)
    }
}

pub async fn delete_user_service(
    db_name: &str,
    data: Arc<MongoRepo>,
    user_id: &str,
) -> Result<(), UserServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<User>("users");

    let id = ObjectId::parse_str(user_id).map_err(|_| UserServiceError::InvalidId)?;

    let filter = doc! { "_id": id };

    let delete_result = collection.delete_one(filter, None).await
        .map_err(|e| UserServiceError::DatabaseError(e.to_string()))?;

    if delete_result.deleted_count == 1 {
        Ok(())
    } else {
        Err(UserServiceError::NotFound)
    }
}

pub async fn get_all_users_service(
    db_name: &str,
    data: Arc<MongoRepo>,
) -> Result<Vec<User>, UserServiceError> {
    let db = data.get_db(db_name).await;
    let collection = db.collection::<User>("users");

    let find_options = FindOptions::builder().build();

    let mut cursor = collection.find(None, find_options).await
        .map_err(|e| UserServiceError::DatabaseError(e.to_string()))?;

    let mut users = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(user) => users.push(user),
            Err(e) => return Err(UserServiceError::DatabaseError(e.to_string())),
        }
    }
    Ok(users)
}
