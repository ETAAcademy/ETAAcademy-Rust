# ETAAcademy-Rust: 02. HTTP

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>02. HTTP</td>
    <td>
      <table>
        <tr>
          <th>rust</th>
          <th>basic</th>
          <td>HTTP</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https:github.com/ETAAcademy)｜[Twitter](https:twitter.com/ETAAcademy)｜[ETA-ZK-Meme](https:github.com/ETAAcademy/ETAAcademy-ZK-Meme)

Authors: [Eta](https:twitter.com/pwhattie), looking forward to your joining

# HTTP server or Webserver

### Message Flow of an HTTP Server

The process of handling an HTTP request begins with the client sending a request, which the server receives. The server uses an HTTP library to process this request, sending it to a router, which then determines the appropriate handler to process it. The handler, again using the HTTP library, processes the request and formulates an HTTP response to send back to the client.

Since Rust lacks built-in HTTP support, this process involves several steps: the server listens for incoming TCP byte streams, the HTTP library parses these streams into HTTP requests and prepares responses, the router directs the requests to the appropriate handlers, and the handlers process the requests and generate responses. The entire communication relies on converting between byte streams and structured HTTP messages.

### HTTP Request

In the `httprequest.rs` file, the HTTP request is parsed using two enums, `Method` and `Version`, and a `HttpRequest` struct. Each of these implements the `From` trait, which is used to process each line of the request. The `HttpRequest` struct also has methods for handling the request line and headers. It derives the `Debug` and `PartialEq` traits for easier debugging and comparison.

The process involves splitting each line of the HTTP request by whitespace and colons. For example, a request might look like this:

```
"GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */_\r\n\r\n"
```

- The first line contains the method, resource, and version.
- The subsequent lines are headers, which are inserted into a hashmap.
- An empty line follows, and then the message body, which is assigned directly.

To begin, create two packages in your workspace: one for the HTTP server (`cargo new httpserver`) and one for the HTTP library (`cargo new --lib http`).

Start by defining the `Method` and `Version` enums and implementing their traits. Then, define the `HttpRequest` struct and implement its traits.

#### 1) Method and Version Enums

- **Variants**: Each enum includes three variants. The `Method` enum has `Get` and `Post` methods, while the `Version` enum has two versions. An `Uninitialized` variant is used for initial setup.
- **Traits**: Both enums derive the `Debug` and `PartialEq` traits and implement the `From` trait to match incoming string slices.

#### 2) HttpRequest Struct

- **Fields**: The struct includes fields for `method`, `version`, `resource` (the request path), `headers` (a key-value collection using a hashmap), and `msg_body` (a `String` for the message body).
- **Traits**: The `From` trait initializes variables and processes each line of the request:
  - The request line (containing "HTTP") is identified as such.
  - Headers containing colons (":") are inserted into the hashmap.
  - The message body is handled by directly assigning the entire line.
  - Finally, the `HttpRequest` is returned.

#### 3) Methods

- **process_req_line**: Handles the request line by splitting it into words using `split_whitespace()`. The first word is the `method`, the second is the `resource`, and the third is the `version`. Each is converted to the appropriate type.
- **process_header_line**: Handles headers by splitting the line using a colon as a delimiter. It assigns `key` and `value`, both as empty strings initially. The `Some` type returns an `Option`.

<details><summary>CODE</summary>

```python
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    UNINITIALIZED,
}
impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::UNINITIALIZED,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    UNINITIALIZED,
}
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::UNINITIALIZED,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::UNINITIALIZED;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}
///GET /greeting HTTP/1.1
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}
///HOST: localhost
///Accept: */*
fn process_header_line(s: &str) -> (String, String){
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next(){
        key = k.to_string();
    }
    if let Some(v) = header_items.next(){
        value = v.to_string();
    }
    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
    #[test]
    fn test_read_http(){
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHOST: localhost\r\nAccept: */*\r\nUser-Agent: Mobile/Iphone");
        let mut header_expected = HashMap::new();
        header_expected.insert("HOST".into(), " localhost".into());
        header_expected.insert("Accept".into(), " */*".into());
        header_expected.insert("User-Agent".into(), " Mobile/Iphone".into());
        let req:HttpRequest = s.into();
        
        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(header_expected, req.headers);

    }
}

```

</details>

### HTTP Response

An HTTP response consists of a status line (HTTP version, status code, and status text), headers (key-value pairs), an empty line, and an optional message body. In Rust, implementing an HTTP response involves creating a struct with methods for setting default values (`Default` trait), creating a new instance (`new()` method), sending the response over TCP (`send_response()` method), retrieving private member values (getter methods), and converting the response to a string (`From` trait).

<details><summary>CODE</summary>

```python

use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}
impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}
impl<'a> From<HttpResponse<'a>> for String{
    fn from(res: HttpResponse<'a>) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {

        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200"{
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };
        response.body = body;
        response
    }
    pub fn send_response(&self, write_stream:&mut impl Write) -> Result<()>{
        let res = self.clone();
        let response_string : String  = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }
    fn version(&self) -> &str{
        self.version
    }
    fn status_code(&self) -> &str{
        self.status_code
    }
    fn status_text(&self) -> &str{
        self.status_text
    }
    fn headers(&self) -> String{
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string : String = "".into();
        for(k , v) in map.iter(){
            header_string = format!("{}{}:{}\r\n", header_string, k , v);
        }
        header_string
    }
    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "".into(),
        }
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_response_struct_creation_200(){
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("xxx".into()),
        );
        let response_expected = HttpResponse{
            version:"HTTP/1.1",
            status_code:"200",
            status_text: "OK",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404(){
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("xxx".into()),
        );
        let response_expected = HttpResponse{
            version:"HTTP/1.1",
            status_code:"404",
            status_text: "Not Found",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }
    #[test]
    fn test_http_response_creation(){
        let response_expected = HttpResponse{
            version: "HTTP/1.1",
            status_code: "404",
            status_text:"Not Found",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        };
        let http_string: String = response_expected.into();
        let actual_string = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 3\r\n\r\nxxx";
        assert_eq!(http_string, actual_string);
    }
}
```

</details>

### Httpserver

**1) Preparation:**

- The `httpserver` needs to reference the `http` package, so set up the dependency relationship as follows:
  ```toml
  [dependencies]
  http = { path = "../http" }
  ```
- Create three modules: `server.rs`, `router.rs`, and `handler.rs`. The flow of execution is as follows: the `main` function calls `server.rs`, which in turn calls the relevant functions from `router.rs`, and `router.rs` calls `handler.rs`.

**2) server.rs**

- The `Server` struct contains a `socket_addr` address, which is a reference type `&'a str`.
- Methods of the `Server` struct:
  - **Associated Method `new`**: Takes a `socket_addr` address as input and returns a `Server` instance.
  - **`run` Method**: Runs the server. It uses `TcpListener::bind` to bind to the address, and the `incoming` method loops through incoming connections. As explained in the TCP section, it reads the data from the stream into a buffer, then converts the buffer's contents into an `HttpRequest` struct (first by calling the `to_vec()` method to convert it to a `String`, then calling `unwrap()`. Since it implements the `From` trait, the `into()` method can be used to convert it into an `HttpRequest`). Finally, it calls the `route` method in the router to dispatch the request to different handlers for processing.

<details><summary>CODE</summary>

```python
use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}
impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buf = [0; 200];
            stream.read(&mut read_buf).unwrap();
            let req: HttpRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
```

</details>

### Router

1. **Initial Request Handling**:
   The router first matches the incoming HTTP request method. Currently, it processes only GET requests, with all other request methods being directed to a `PageNotFoundHandler`.

2. **Path Parsing**:
   If the request method is GET, the request path (resource), stored as a `String`, is split into a vector of strings using slashes (`/`) as delimiters. This helps in breaking down the resource path for further analysis.

3. **Route Matching**:
   The router examines the second element of the split path. If this element is `"api"`, it categorizes the request as a web service request, routing it to a `WebServiceHandler`. For all other cases, it assumes the request is for a static page and sends it to a `StaticPageHandler`.

4. **JSON Support**:
   To facilitate processing of JSON data, serialization, and deserialization within the handlers, the project includes dependencies on `serde` and `serde_json`. These libraries enable efficient handling of JSON in the application.

<details><summary>CODE</summary>

```python

use crate::handler::{WebServiceHandler, StaticPageHandler};
use super::handler::{Handler, PageNotFoundHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;
impl Router{
    pub fn route(req: HttpRequest, stream:&mut impl Write) -> (){
        match req.method {
            httprequest::Method::GET => match  &req.resource{
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        //localhost:3000/api
                        "api" => {
                            let resp : HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _=>{
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp:HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }

    }
}

```

</details>

### Handlers

1. **Handling Web Requests**:

   - A `handle` function processes `HttpRequest` and returns `HttpResponse`.

2. **Loading Files**:

   - `load_file` function handles static HTML files, returning `Option<String>`.
   - Uses environment variables to determine the project's root directory and construct file paths.
   - Reads file content using `fs::read_to_string`.

3. **Implementing Handlers**:

   - **PageNotFoundHandler**: Returns 404 response and loads a static 404 page.
   - **StaticPageHandler**:
     - Handles requests by checking the request path.
     - Returns `index.html` for root paths and `health.html` for health check paths.
     - Reads file content based on path and sets appropriate content types (e.g., `text/css`, `text/javascript`).
     - Returns 404 if path does not match any file.
   - **WebServiceHandler**:
     - Loads JSON data from files, processes it, and returns it as `HttpResponse`.
     - Handles specific API paths for JSON data and returns JSON content type.
     - Returns 404 for non-matching paths.

4. **Defining Business Logic for JSON Files**:
   - Creates `OrderStatus` struct for representing order data.
   - Implements serialization and deserialization for JSON processing.

<details><summary>CODE</summary>

```python

use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req:&HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String>{
        //CARGO_MANIFEST_DIR crate 根目录
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        //PUBLIC_PATH package根目录
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus{
    order_id:i32,
    order_date:String,
    order_status:String,
}

impl Handler for PageNotFoundHandler {
    fn handle(req:&HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
impl Handler for StaticPageHandler {
    fn handle(req:&HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        //localhost:300/health/api
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(content) => {
                    let mut map : HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css"){
                        map.insert("Content-Type", "text/css");
                    }else if path.ends_with(".js"){
                        map.insert("Content-Type", "text/javascript");
                    }else{
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(content))
                },
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            }

        }
    }
}
impl WebServiceHandler{
    fn load_json() -> Vec<OrderStatus>{
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
        serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}
impl Handler for WebServiceHandler{
    fn handle(req:&HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        //localhost:3000/api/shipping/orders
        let route:Vec<&str> = s.split("/").collect();
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers:HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            },
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}

```

</details>
