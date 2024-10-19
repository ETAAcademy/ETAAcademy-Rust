# ETAAcademy-Rust: 03. WebAPI

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>03. WebAPI</td>
    <td>
      <table>
        <tr>
          <th>rust</th>
          <th>basic</th>
          <td>WebAPI</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https:github.com/ETAAcademy)｜[Twitter](https:twitter.com/ETAAcademy)｜[ETA-ZK-Rust](https://github.com/ETAAcademy/ETAAcademy-Rust)

Authors: [Eta](https:twitter.com/pwhattie), looking forward to your joining

# Building a Web API with Actix in Rust

### Actix Web Framework Overview

**Actix** is a powerful, concurrent web framework in Rust. Here's an overview of its basic components and how it works:

1. **Request Flow**: When a client sends a request, it reaches the Actix HTTP Server. This server forwards the request to a specific route within an Actix App (e.g., GET /health). An Actix Handler function (like `health_check_handler`) then processes the request and generates a response.

2. **Concurrency**: Actix uses a combination of asynchronous I/O and multithreading for efficient handling of concurrent requests:
   - **Asynchronous I/O**: An OS-native thread performs tasks while waiting for network operations (e.g., listening for network connections).
   - **Multithreading**: By default, Actix launches multiple threads equal to the number of CPU cores for parallel processing.

### Web API Project

To build a Web API using Actix, you'll need to add external dependencies for the `actix-web` framework and its asynchronous runtime library, `actix-rt`. The `actix-rt` library provides the async runtime environment for `actix-web`, enabling you to write asynchronous network applications in Rust.

1. **Create a New Workspace**:

   - Run `cargo new ws` to create a new workspace named `ws`.
   - In the `Cargo.toml` file of `ws`, convert the project into a workspace.
     - Add a subproject named `webservice` by running `cargo new webservice` inside the `ws` directory.
     - Add dependencies for `actix-web` and `actix-rt`.
   - Build the project with `cargo build`. If successful, specify a binary name using the `[[bin]]` section (this allows for multiple binaries, such as `server1`, `server2`, etc., but we'll only add one for now).

   ```toml
    [workspace]
    members = ["webservice"]
   ```

   ```toml
   [package]
    name = "webservice"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    actix-web = "4"
    actix-rt = "2.7.0"

    [[bin]]
    name = "server1"
   ```

2. **Create and Configure the Server**:

   - Inside the `src` directory of `webservice`, create a `bin` directory and add `server1.rs`, which will contain the main function for the `server1` binary.

3. **Code Implementation**:

   - **Import necessary components**: In `server1.rs`, import `web`, `App`, `HttpResponse`, `HttpServer`, and the `Responder` trait from `actix-web`.
   - **Define the `main` function**: Define an async main function to run the server. Initialize an `HttpServer`, passing the configured Web App and binding it to port 3000.
     - **General Routes Function**: Create an Actix App and configure routes using the `general_routes` function. This function configures a route, `/health`, which uses the `GET` method and maps to the `health_check_handler`.
     - **Health Check Handler**: This async function returns a `200 OK` response with a JSON message indicating that the web service is healthy. The response needs to implement the Responder trait.

4. **Running the Server**:

   - **Method 1**:
     - Navigate to the ws directory.
     - Run cargo run -p webservice --bin server1.
     - Open a browser and go to http://localhost:3000/health. You should see "Actix Web Server is running!".
   - **Method 2**: If you're already in the webservice directory, run cargo run --bin server1.

   ```rust

        use actix_web::{web, App, HttpResponse, HttpServer, Responder};
        use std::io;

        pub fn general_routes(cfg: &mut web::ServiceConfig) {
            cfg.route("/health", web::get().to(health_check_handler));
        }

        pub async fn health_check_handler() -> impl Responder {
            HttpResponse::Ok().json("Actix Web Server is running!")
        }

        #[actix_rt::main]
        async fn main() -> io::Result<()> {
            let app = move ||  App::new().configure(general_routes);

            HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
        }

   ```

This setup provides a basic framework for developing a web API with Actix, offering both robust concurrency and scalability.
