# ETAAcademy-Rust: 01. TCP

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>01. TCP</td>
    <td>
      <table>
        <tr>
          <th>rust</th>
          <th>basic</th>
          <td>TCP</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https:github.com/ETAAcademy)｜[Twitter](https:twitter.com/ETAAcademy)｜[ETA-ZK-Rust](https://github.com/ETAAcademy/ETAAcademy-Rust)

Authors: [Eta](https:twitter.com/pwhattie), looking forward to your joining

# Building a Simple TCP Server and Client in Rust

This guide walks you through creating a basic TCP server and client application in Rust using the `std::net` module. In this section, we will focus on using two main data structures: `TcpListener` and `TcpStream`. First, let's set up the project.

**Project Setup**

Workspace Creation: We'll use a workspace `s1` to manage both the server and client code as separate projects within a single unit. Here's the workspace configuration file (Cargo.toml):

```rust
[workspace]

members = ["tcpserver", "tcpclient"]
```

### TCP Server

The server listens for incoming connections and echoes back received messages.

**Listening for Connections:** On the server side, `TcpListener::bind` listens for incoming connections, each represented as a stream of bytes (`TcpStream`), which is used to send and receive data. The client establishes a connection and returns a stream using the `TcpStream::connect` function. The `TcpListener::bind` function is used to create a listener by passing in an IP address and port, in this case, binding to the local port 3000.

**Accepting Connections:** To handle TCP connections in Rust, the `accept` method can be used to accept a single connection, returning a `Result` with a tuple containing a `TcpStream` and a `SocketAddr`. For continuous listening, it's more common to use the `incoming` method with a loop, which returns an iterator to listen for connections. Each connection is represented as a `TcpStream`, allowing for reading and writing of raw byte data, typically using byte arrays like `&[u8]` or `Vec<u8>`. Rust also supports byte strings for working with binary data, and `TcpStream` implements the `Read` and `Write` traits for data transmission.

**Reading and Writing Data:**

- Read and Write Traits: TcpStream implements these traits, allowing us to read and write data.
- stream.read: Reads data sent by the client into a buffer (e.g., [0; 1024]).
- stream.write: Sends the received data back to the client.

```rust
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000 ...");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
```

### TCP Client

The client connects to the server, sends a message, and receives the server's response.

- Connecting to Server: `TcpStream::connect` establishes a connection to the server's address and port.

- Sending Message: `.as_bytes()` converts the message string ("Hello") into raw bytes for transmission.

- Receiving Response: `stream.read` reads the server's response into a buffer. `str::from_utf8` converts the received bytes into a readable string.

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000 ...");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}

```
