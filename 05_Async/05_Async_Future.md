# ETAAcademy-Rust: 05. Async & Future

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>05. Async & Future</td>
    <td>
      <table>
        <tr>
          <th>rust</th>
          <th>basic</th>
          <td>Async_Future</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https:github.com/ETAAcademy)｜[Twitter](https:twitter.com/ETAAcademy)｜[ETA-ZK-Rust](https://github.com/ETAAcademy/ETAAcademy-Rust)

Authors: [Eta](https:twitter.com/pwhattie), looking forward to your joining

# Concurrency in Rust: Understanding Asynchronous Programming, Multithreading, and Futures

In modern software development, handling multiple tasks efficiently is crucial for performance, especially when dealing with large-scale web applications or systems that manage numerous requests. Rust, with its powerful and safe concurrency model, provides several ways to handle such challenges. In this article, we'll explore **concurrency**, **multithreading**, and **asynchronous programming** in Rust, focusing on how they work and differ from each other.

### Concurrency: A Brief Overview

Concurrency refers to the ability of different parts of a program to execute independently and potentially in parallel. It allows tasks to progress without waiting for the completion of other tasks. In the context of web applications, for instance, each HTTP request is handled by an asynchronous web server, which spawns an **async task** for each request. The **asynchronous runtime** schedules these tasks to run on available CPUs, optimizing resource use. This ability to handle multiple tasks simultaneously is also known as **parallelism** when tasks are executed at the same time.

### Synchronous vs. Multithreading vs. Asynchronous Programming

To better understand Rust's approach to concurrency, let's consider a task with three steps:

1. Data processing
2. A blocking operation
3. Packaging the results for return

Each programming model approaches this task differently.

**Synchronous Programming**

In **synchronous execution**, the program follows a strict sequence. First, Task 1 is executed, followed by the blocking operation. After the operation completes, Task 1 finishes, and then the program moves on to Task 2, Task 3, and so on.

**Multithreading**

In **multithreading**, multiple threads are created to handle different tasks simultaneously. For instance, **Thread 1** might execute Task 1 to completion, while **Thread 2** handles Task 2 and Task 3. However, multithreading introduces challenges such as unpredictable execution order, potential deadlocks, and **race conditions**. Rust offers two primary models for multithreading:

- **1:1 Model**: Each language thread corresponds to a system thread. Rust’s standard library follows this model. Although straightforward, this model has limitations. The operating system often restricts the number of threads due to memory and resource constraints, and frequent **context switching** between threads can incur significant overhead.
- **M:N Model**: In this model, multiple user-level (green) threads are mapped to a limited number of system threads. This approach can improve efficiency by reducing the number of system-level threads in use.

**Asynchronous Programming**

In **asynchronous programming**, a single thread can handle multiple tasks without waiting for blocking operations to complete. For example, **Thread 1** can begin Task 1, perform a blocking operation, and instead of waiting, continue executing other parts of the task or move on to another task entirely. Once the blocking operation completes, the thread resumes Task 1.

Rust’s asynchronous model, built around the **async/await** paradigm, allows for highly efficient handling of I/O-bound tasks by letting the runtime handle blocking operations in the background.

### How Async Works in Rust

Rust’s async functionality is powered by asynchronous runtimes, with **Tokio** being one of the most popular. The `#[tokio::main]` attribute tells the Rust compiler to use Tokio as the runtime. By adding the `async` keyword, you enable functions to become asynchronous tasks, which the Tokio runtime manages and schedules. When using `tokio::spawn`, new asynchronous tasks are created. Depending on the runtime’s configuration, these tasks may execute on a single thread or multiple threads. To wait for multiple async tasks to complete, the `tokio::join!` macro is used.

A key point to remember is that asynchronous functions in Rust are **lazy**—they don’t begin execution until the `.await` keyword is encountered. This ensures that the function’s execution is paused and resumed as needed by the runtime, optimizing CPU and resource usage.

<details><summary>Code</summary>

```rust
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Hello before reading file!");

    let h1 = tokio::spawn( async {
        let _file1_contents = read_from_file1().await;
    });

    let h2 = tokio::spawn( async {
        let _file2_contents = read_from_file2().await;
    });
    let _ = tokio::join!(h1, h2);
}

async fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    println!("{:?}", "Processing file 1");
    String::from("Hello, there from file 1")
}

async fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    println!("{:?}", "Processing file 2");
    String::from("Hello, there from file 2")

```

</details>

### The Role of Futures in Rust

At the core of Rust’s asynchronous programming is the **Future** type. A `Future` represents a deferred computation that eventually produces a single result. Every async function in Rust returns a `Future`, which effectively describes an operation that hasn’t completed yet. **The asynchronous executor**, a key component of the async runtime, manages a collection of these `Future`s. The executor periodically **polls** the `Future` by calling its `poll` method, which drives it toward completion.

When you declare an async function or block in Rust, you’re essentially informing the executor that this function returns a `Future`. The executor must keep polling the `Future` until it finishes.

In Rust, the `Future` trait has an `Output` type, which defines the result type produced when the `Future` completes. For example, an async function returning a `String` would have `Output = String`. The Rust runtime ensures that the `Future` is driven to completion by calling `poll`.

**Understanding Wakers and Pinning**

In Rust’s async model, two additional components play important roles: **Wakers** and **Pinning**.

- **Waker**: A `Waker` is responsible for notifying the async runtime when a task is ready to resume. If a `Future` is not yet ready to produce a result, it registers with a `Waker` and returns `Poll::Pending`. The `Waker` will then notify the runtime to poll the `Future` again when the necessary conditions are met (such as I/O completion).

- **Pin**: `Pin` ensures that data is "pinned" in memory and cannot be moved. This is crucial because async tasks are polled repeatedly, and moving them in memory during execution could result in undefined behavior. By using `Pin`, Rust ensures that `Future`s remain in a stable memory location throughout their execution.

<details><summary>Code</summary>

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::Duration;

struct ReadFileFuture {}

impl Future for ReadFileFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // println!("Tokio! Stop polling me");
        // cx.waker().wake_by_ref();
        // Poll::Pending

        println!("Tokio! Stop polling me");
        cx.waker().wake_by_ref();
        Poll::Ready(String::from("Hello, there from file 1"))

    }
}

#[tokio::main]
async fn main() {
    println!("Hello before reading file!");

    let h1 = tokio::spawn( async {
        let future1 = ReadFileFuture {};
        future1.await
    });

    let h2 = tokio::spawn( async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
    });
    let _ = tokio::join!(h1, h2);
}

// async fn read_from_file1() -> String {
//     sleep(Duration::new(4, 0));
//     println!("{:?}", "Processing file 1");
//     String::from("Hello, there from file 1")
// }

// async fn read_from_file2() -> String {
//     sleep(Duration::new(2, 0));
//     println!("{:?}", "Processing file 2");
//     String::from("Hello, there from file 2")
// }

fn read_from_file1() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(4, 0));
        println!("{:?}", "Processing file 1");
        String::from("Hello, there from file 1")
    }
}

fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(3, 0));
        println!("{:?}", "Processing file 2");
        String::from("Hello, there from file 2")
    }
}

```

</details>

### Building a Custom Future: An Example

To demonstrate how Futures work in Rust, let’s consider creating a custom `Future`—a timer that performs the following tasks:

1. It allows setting a timeout duration.
2. When polled by the async executor, it checks:
   - If the current time is greater than or equal to the timeout, it returns `Poll::Ready` along with a `String` value.
   - If the current time is less than the timeout, it sleeps until the timeout is reached. At this point, it triggers the `Waker`, which notifies the async runtime to resume execution and poll the `Future` again.

This custom `Future` behaves like a timer that waits for a specific duration before completing and returning its result. By implementing the `Future` trait and utilizing the `poll` method, you can drive this `Future` to completion, integrating it seamlessly into Rust’s asynchronous ecosystem.

<details><summary>Code</summary>

```rust

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::{Duration, Instant};

struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<' _>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("Hello, it's time for Future 1");
            Poll::Ready(String::from("Future 1 has completed"))
        }else{
            println!("Hello, it's not yet time for Future 1. Going to sleep");
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            std::thread::spawn(move || {
                let current_time = Instant::now();
                if current_time < expiration_time {
                    std::thread::sleep(expiration_time - current_time);
                }
                waker.wake();
            });
            Poll::Pending
        }




    }
}

#[tokio::main]
async fn main() {
    let h1 = tokio::spawn( async {
        let future1 = AsyncTimer {
            expiration_time: Instant::now() + Duration::from_millis(4000),
        }
        println!("{:?}", future1.await);
    });

    let h2 = tokio::spawn( async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
    });
    let _ = tokio::join!(h1, h2);
}

fn read_from_file1() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(2, 0));
        String::from("Future 2 has completed")
    }
}

fn read_from_file2() -> impl Future<Output = String> {
    async {
        sleep(Duration::new(3, 0));
        println!("{:?}", "Processing file 2");
        String::from("Hello, there from file 2")
    }
}

```

</details>

### Conclusion

Concurrency and asynchronous programming are essential for building scalable and efficient systems, especially when handling numerous concurrent tasks. Rust’s approach, built around **async/await** and **Futures**, provides a powerful yet safe model for handling concurrency.

By leveraging runtimes like Tokio and understanding key concepts like **Wakers** and **Pinning**, developers can build efficient, non-blocking systems that maximize performance.

<div style="text-align: center;">
    <img src="./05_async_future.webp" alt="Image 1" width="100%" style="display: inline-block;">
</div>
