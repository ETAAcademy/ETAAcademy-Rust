# ETAAcademy-Rust: 04. WebService

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>04. WebService</td>
    <td>
      <table>
        <tr>
          <th>rust</th>
          <th>basic</th>
          <td>WebService</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https:github.com/ETAAcademy)｜[Twitter](https:twitter.com/ETAAcademy)｜[ETA-ZK-Rust](https://github.com/ETAAcademy/ETAAcademy-Rust)

Authors: [Evta](https:twitter.com/pwhattie), looking forward to your joining

# Rust-based Webservice for Resource Management

This backend service, built using Rust, provides a set of APIs for managing resources like courses. It involves setting up a web server, defining endpoints for various operations, handling requests, interacting with a database, and generating appropriate responses.

### 1. **Web Server Setup**:

- Using a web framework like `Actix-web` to set up the server and define routes.
- Create a project: It should use two external crates, serde (a library for serialization and deserialization) and chrono (for time-related fields). Set the default-run to "teacher-service" (i.e., the default binary to run if none is specified).
- Goal: Build a REST API that can add courses, retrieve a specific course, and all courses of a teacher.

```
webservice/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── dbaccess/
│   │   └── course.rs
|   |   └── mod.rs
|   |   └── teacher.rs
│   ├── handles/
│   │   └── course.rs
│   │   └── general.rs
│   │   └── mod.rs
│   │   └── teacher.rs
│   ├── models/
│   │   └── course.rs
│   │   └── mod.rs
│   │   └── teacher.rs
│   ├── errors.rs
│   ├── routers.rs
│   └── state.rs
└── .env
```

### 2. **Routing**: Defining endpoints for different operations (e.g., CRUD operations for courses).

In a typical Rust web service project, files like `state.rs`, `routers.rs`, and `handlers.rs` serve specific purposes.

**`state.rs`**: Manages shared application state, such as database connections.

Using the Actix framework, It usually contains the application's shared state, which might include database connection pools, configuration settings, or other shared resources, as well as inject state into the Handler (the Handler can access the content in the state through the parameters in the method signature).

<details><summary>Example</summary>

```rust

use std::sync::Mutex;
use sqlx::PgPool;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub db: PgPool
}

```

</details>

**`routers.rs`**: Configures the routing for the web service, mapping endpoints to handlers.

Similarly, router typically defines the routing configuration for the web service. It maps HTTP endpoints to their corresponding handler functions.the `general_routes` method configures the route. The argument type is `web::ServiceConfig`. When configuring routes, the first part is the path, and `web::get()` specifies the HTTP method. For example, `cfg.route("/health", web::get().to(health_check_handler));` defines a route where the path is `/health`, the method is HTTP GET, and `health_check_handler` is the corresponding handler.

<details><summary>Example</summary>

```rust:src/routers.rs
use crate::handlers::{general::*, course::*, teacher::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
            .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
            .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
            .route("/{teacher_id}/{course_id}", web::put().to(update_course_detail))
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/teachers")
            .route("/", web::post().to(post_new_teacher))
            .route("/", web::get().to(get_all_teachers))
            .route("/{teacher_id}", web::get().to(get_teacher_details))
            .route("/{teacher_id}", web::put().to(update_teacher_details))
            .route("/{teacher_id}", web::delete().to(delete_teacher)),
    );
}
```

</details>

**`handlers.rs`**: Contains the logic for handling HTTP requests, interacting with the database, and returning responses.

This part contains the actual handler functions that process incoming HTTP requests. These functions typically interact with the database and return responses and inject the data into the handler to access it directly. The `lock` method is used to prevent other threads from updating the value simultaneously to finally forms a response.

<details><summary>Example</summary>

```rust:src/handlers.rs
use crate::AppState;
use crate::dbaccess::course::*;
use crate::errors::MyError;
use actix_web::{web, HttpResponse};
use crate::models::course::{CreateCourse, UpdateCourse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_course_details_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_detail(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_detail_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

```

</details>

### 3. **Database Interaction**:

Database is a crucial part of the backend logic. The `dbaccess` module typically contains functions that interact with the database, performing CRUD (Create, Read, Update, Delete) operations. These functions are often called from handler functions to process incoming HTTP requests and Database interaction involves:

1. **Connecting to the Database**: Using an ORM or SQL toolkit like `sqlx` to interact with a database and establishing a connection to the database using a connection pool.
2. **Executing Queries**: Running SQL queries to fetch, insert, update, or delete data.
3. **Handling Results**: Processing the results of the queries and handling any errors that may occur.

**Add code to read environment variables**

1. `dotenv().ok()` is used to load environment variables from the `.env` file and then the `env::var` function reads the value of the environment variable `"DATABASE_URL"` and assigns it to `database_url`.
2. Create a database connection pool (Postgres connection pool), while initializing `AppState`, replace it with the database properties.

**Database operations**

- `get`function retrieves all courses for a specific teacher. The `sqlx::query!` macro is used to prepare SQL queries,in which `r##` is used to represent multiline queries, and `$1` serves as a placeholder for parameters (e.g., `$1`, `$2`, `$3` correspond to the 3 values of `new_course`).
- The `time` field doesn't need to be written because the database has a default value for it, which is the current date via the `now()` function.
- The `fetch_all` function is used to query multiple records, while `fetch_one` is used for a single record, both of which execute SQL queries. The function takes the database connection pool (`PgPool`) as its parameter and `await` is used for asynchronous execution.
- The `iter` method is used to iterate over the returned courses, and since `time` is of type `Option`, it’s handled with `Some`. Finally, `collect` is used to convert the result into a vector.

<details><summary>Example</summary>

```rust:src/dbaccess/course.rs
use sqlx::postgres::PgPool;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use crate::errors::MyError;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM courses WHERE teacher_id = $1"#,
        teacher_id
    )
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM courses
        WHERE teacher_id = $1 AND id = $2"#,
        teacher_id,
        course_id
    )
        .fetch_optional(pool)
        .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO courses (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#,
        new_course.teacher_id,new_course.name,new_course.description,
        new_course.format,new_course.structure,new_course.duration,
        new_course.price,new_course.language,new_course.level,
    )
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub async fn delete_course_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        "DELETE FROM courses where teacher_id = $1 and id=$2",
        teacher_id,
        course_id,
    )
        .execute(pool)
        .await?;
    Ok(format!("DeletedI{:?}record", course_row))
}

pub async fn update_course_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM courses WHERE teacher_id = $1 AND id = $2"#,
        teacher_id,
        course_id
    )
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let description: String = if let Some(desc) = update_course.description {
        desc
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };

    let course_row = sqlx::query_as!(
        Course,
        "UPDATE courses
        SET name = $1, description = $2, format = $3,
        structure = $4, duration = $5, price = $6, language = $7,
        level = $8 WHERE teacher_id = $9 AND id = $10
        RETURNING id, teacher_id, name, time,
        description, format, structure, duration, price, language, level",
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        course_id,
    )
        .fetch_one(pool)
        .await;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}

```

</details>

### 4. **Models**: Defining data structures that represent the entities in application (e.g., `Course`, `Teacher`).

In a Rust web service project, the `models` module typically contains the data structures that represent the entities in your application. These structures are often used to map database records to Rust types, making it easier to work with the data in your application:

1. **Data Representation**: Models represent the structure of data, such as tables in a database.
2. **Usage**: Used in database access functions and handlers to manage data flow in the application.
3. **Serialization/Deserialization**: Models are often used for serializing data to and from formats like JSON, which is useful for API responses and requests.

<details><summary>Example</summary>

```rust:src/models/course.rs
use crate::errors::MyError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;

    fn try_from(course: web::Json<CreateCourse>)
                -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}

```

</details>

### 5. **Error Handling**: Implementing custom error types and handling errors gracefully.

In Rust, error handling is typically done using the `Result` type, which can represent either a success (`Ok`) or an error (`Err`). Custom error types are often defined to provide more context and control over the errors that can occur in the application.

1. **Custom Error Enum**:

   - `MyError` enum defines different types of errors that can occur in the application.
   - Variants include `DbError` for database errors, `NotFound` for resource not found errors, and `InternalServerError` for generic server errors.

2. **Implementing `std::error::Error`**:

   - This trait is implemented to make `MyError` compatible with Rust's error handling ecosystem.

3. **Implementing `ResponseError`**:

   - The `ResponseError` trait from `actix_web` is implemented to convert `MyError` into HTTP responses.
   - The `error_response` method maps each error variant to an appropriate HTTP status code and message.

4. **Usage in the Project**:

   - **Database Access Functions**: Return `Result<T, MyError>` to propagate errors up the call stack.
   - **Handlers**: Handle errors returned from database access functions and convert them into HTTP responses using the `ResponseError` implementation.

<details><summary>Example</summary>

```rust:src/errors.rs
use actix_web::{error, Error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;
use std::fmt::{Display, Formatter};
use actix_web::body::BoxBody;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            MyError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(error: Error) -> Self {
        MyError::ActixError(error.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(error: SQLxError) -> Self {
        MyError::DBError(error.to_string())
    }
}

```

**Example Usage in `dbaccess/course.rs`**

```rust:src/dbaccess/course.rs
use sqlx::PgPool;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use crate::errors::MyError;

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let courses = sqlx::query_as!(
        Course,
        "SELECT * FROM courses WHERE teacher_id = $1",
        teacher_id
    )
    .fetch_all(pool)
    .await
    .map_err(MyError::DbError)?;
    Ok(courses)
}

// Other functions follow a similar pattern
```

**Example Usage in `handlers.rs`**

```rust:src/handlers.rs
use actix_web::{web, HttpResponse, Result};
use crate::state::AppState;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use crate::dbaccess::course::{get_courses_for_teacher_db, get_course_details_db, post_new_course_db, delete_course_db, update_course_detail_db};
use crate::errors::MyError;

pub async fn get_courses(state: web::Data<AppState>, teacher_id: web::Path<i32>) -> Result<HttpResponse, MyError> {
    let courses = get_courses_for_teacher_db(&state.db_pool, teacher_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(courses))
}

// Other handlers follow a similar pattern
```

</details>

### 6. **Testing**: Writing tests to ensure the functionality of endpoints and database operations.

Testing ensures code works as expected and helps prevent regressions. In Rust, testing is built into the language and the standard library, making it straightforward to write and run tests.

- **Unit Tests**: Test individual functions or modules in isolation.
- **Integration Tests**: Test the interaction between multiple components or modules.
- **End-to-End Tests**: Test the entire application from start to finish, often simulating user interactions.
- **Running Tests**: Use `cargo test` to run all tests.

**Unit Tests**

Unit tests are typically placed in the same file as the code they are testing, within a `#[cfg(test)]` module.

<details><summary>Example</summary>

```rust:src/models/course.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_course() {
        let new_course = CreateCourse {
            teacher_id: 1,
            name: String::from("Rust Programming"),
            description: Some(String::from("Learn Rust")),
            format: Some(String::from("Online")),
            structure: Some(String::from("Modules")),
            duration: Some(String::from("10 weeks")),
            price: Some(100),
            language: Some(String::from("English")),
            level: Some(String::from("Beginner")),
        };

        assert_eq!(new_course.name, "Rust Programming");
        assert_eq!(new_course.teacher_id, 1);
    }
}
```

</details>

**Integration Tests**

Integration tests are placed in the `tests` directory and can test the interaction between multiple components.

<details><summary>Example</summary>

```rust:tests/integration_tests.rs
use actix_web::{test, App};
use rust_wasm_main::handlers::{get_courses, create_course};
use rust_wasm_main::state::AppState;
use rust_wasm_main::config::create_pool;

#[actix_rt::test]
async fn test_get_courses() {
    let pool = create_pool("postgres://user:password@localhost/test_db").await.unwrap();
    let state = AppState::new(pool);
    let mut app = test::init_service(App::new().data(state).configure(rust_wasm_main::routers::init)).await;

    let req = test::TestRequest::get().uri("/courses").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_create_course() {
    let pool = create_pool("postgres://user:password@localhost/test_db").await.unwrap();
    let state = AppState::new(pool);
    let mut app = test::init_service(App::new().data(state).configure(rust_wasm_main::routers::init)).await;

    let new_course = CreateCourse {
        teacher_id: 1,
        name: String::from("Rust Programming"),
        description: Some(String::from("Learn Rust")),
        format: Some(String::from("Online")),
        structure: Some(String::from("Modules")),
        duration: Some(String::from("10 weeks")),
        price: Some(100),
        language: Some(String::from("English")),
        level: Some(String::from("Beginner")),
    };

    let req = test::TestRequest::post()
        .uri("/courses")
        .set_json(&new_course)
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
```

</details>

<div style="text-align: center;">
    <img src="../04_WebService/images/crab007.webp" alt="Image 1" width="100%" style="display: inline-block;">
</div>
