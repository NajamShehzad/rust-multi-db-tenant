# rust-multi-db-tenant

A **Rust** application demonstrating dynamic multi-database (multi-tenant) handling, built using the high-performance **Actix-Web** framework. This project shows how to:

- Dynamically select a MongoDB database based on an HTTP request header (`_db`).
- Cache and flush database connections after a specified time period (15 minutes of idle time).
- Structure Rust code in a modular way for ease of maintenance and scalability.

## Table of Contents

1. [Overview](#overview)  
2. [Key Features](#key-features)  
3. [Project Structure](#project-structure)  
4. [How It Works](#how-it-works)  
    - [Multiple Database Handling](#multiple-database-handling)  
    - [Database Flushing](#database-flushing)  
5. [Setup and Usage](#setup-and-usage)  
    - [Prerequisites](#prerequisites)  
    - [Installation](#installation)  
    - [Running the Application](#running-the-application)  
6. [API Endpoints](#api-endpoints)  
    - [User Module](#user-module)  
    - [Todo Module](#todo-module)  
7. [License](#license)  

---

## Overview

This example demonstrates how to build a **multi-tenant** CRUD REST API in Rust. Tenants (databases) are chosen dynamically at runtime via an HTTP request header. This allows you to route multiple clients (or tenants) to separate databases without duplicating code or spinning up multiple servers.

## Key Features

- **Dynamic Database Selection**: The `_db` header determines which MongoDB database to use for each request.
- **Caching with TTL**: Uses [moka](https://docs.rs/moka/latest/moka/) to cache database connections and automatically remove them after 15 minutes of inactivity.
- **Modular Architecture**: Separate modules for `user` and `todo` data models, each with its own router, handler, model, service, and error definitions.
- **Async MongoDB Driver**: Non-blocking, highly performant database interactions.
- **Separation of Concerns**: Services return standard Rust `Result` types, while handlers map those results to HTTP responses. This design makes the code more testable and maintainable.

## Project Structure

```
rust-multi-db-tenant/
├── Cargo.toml
├── .env
└── src
    ├── main.rs
    ├── db.rs
    ├── routes
    │   └── mod.rs
    ├── user
    │   ├── mod.rs
    │   ├── user_router.rs
    │   ├── user_handler.rs
    │   ├── user_model.rs
    │   ├── user_service.rs
    │   └── user_errors.rs
    └── todo
        ├── mod.rs
        ├── todo_router.rs
        ├── todo_handler.rs
        ├── todo_model.rs
        ├── todo_service.rs
        └── todo_errors.rs
```

**Highlights**:

- **`src/db.rs`**: Manages database connectivity and caching logic.  
- **`src/routes/mod.rs`**: Central place for registering all module routes.  
- **`src/user/`** and **`src/todo/`**: Each module has its own routing, handlers, models, services, and error definitions.

## How It Works

### Multiple Database Handling

1. **Header-based DB Selection**: Each HTTP request can include a header named `_db`.  
2. **Cache Lookup**: If the requested database (e.g., `"tenant1_db"`) is in the cache, reuse the existing client. Otherwise, create a new `Database` instance.  
3. **Separate Collections**: Each database can have its own `users`, `todos`, etc., collections.

### Database Flushing

- **Moka Cache**: The project uses [moka::future::Cache](https://docs.rs/moka/latest/moka/future/struct.Cache.html) with a **time-to-idle** (`time_to_idle`) set to **15 minutes**.  
- **Idle Expiration**: If no requests are made for a particular database within 15 minutes, its entry automatically expires from the cache. This frees up memory and ensures inactive tenants don’t linger.

## Setup and Usage

### Prerequisites

- **Rust** (Edition 2021 or newer)
- **Cargo** (bundled with Rust)
- **MongoDB** running locally or accessible via network

### Installation

1. **Clone the repo**:

   ```bash
   git clone https://github.com/yourusername/rust-multi-db-tenant.git
   cd rust-multi-db-tenant
   ```

2. **Configure environment**:  
   - Create a `.env` file in the project root (if not present):

     ```bash
     MONGODB_URI=mongodb://localhost:27017
     ```

   - You can also modify the TTL in `db.rs` if needed.

3. **Install dependencies**:

   ```bash
   cargo build
   ```

### Running the Application

```bash
cargo run
```

The server will start on **`127.0.0.1:8080`** by default.

## API Endpoints

### User Module

- **Create User**:  
  `POST /users`  

  ```bash
  curl -X POST \
       -H "Content-Type: application/json" \
       -H "_db: tenant1_db" \
       -d '{"name":"Alice","email":"alice@example.com","password":"password123"}' \
       http://127.0.0.1:8080/users
  ```

- **Get All Users**:  
  `GET /users`  

  ```bash
  curl -H "_db: tenant1_db" http://127.0.0.1:8080/users
  ```

- **Get User by ID**:  
  `GET /users/{id}`  

  ```bash
  curl -H "_db: tenant1_db" http://127.0.0.1:8080/users/63f2a...
  ```

- **Update User**:  
  `PUT /users/{id}`  

  ```bash
  curl -X PUT \
       -H "Content-Type: application/json" \
       -H "_db: tenant1_db" \
       -d '{"name":"Alice Updated","email":"alice@newdomain.com","password":"NewPass"}' \
       http://127.0.0.1:8080/users/63f2a...
  ```

- **Delete User**:  
  `DELETE /users/{id}`  

  ```bash
  curl -X DELETE \
       -H "_db: tenant1_db" \
       http://127.0.0.1:8080/users/63f2a...
  ```

### Todo Module

- **Create Todo**:  
  `POST /todos`  

  ```bash
  curl -X POST \
       -H "Content-Type: application/json" \
       -H "_db: tenant1_db" \
       -d '{"title":"Buy groceries","description":"Milk, Eggs, Bread","completed":false,"user_id":"63f2a..."}' \
       http://127.0.0.1:8080/todos
  ```

- **Get All Todos**:  
  `GET /todos`  

  ```bash
  curl -H "_db: tenant1_db" http://127.0.0.1:8080/todos
  ```

- **Get Todo by ID**:  
  `GET /todos/{id}`  

  ```bash
  curl -H "_db: tenant1_db" http://127.0.0.1:8080/todos/63f2a...
  ```

- **Update Todo**:  
  `PUT /todos/{id}`  

  ```bash
  curl -X PUT \
       -H "Content-Type: application/json" \
       -H "_db: tenant1_db" \
       -d '{"title":"Buy more groceries","description":"Add cereal","completed":false,"user_id":"63f2a..."}' \
       http://127.0.0.1:8080/todos/63f2a...
  ```

- **Delete Todo**:  
  `DELETE /todos/{id}`  

  ```bash
  curl -X DELETE \
       -H "_db: tenant1_db" \
       http://127.0.0.1:8080/todos/63f2a...
  ```

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

Feel free to open a Pull Request if you have any improvements or to file an Issue if you encounter any problems. Happy coding!