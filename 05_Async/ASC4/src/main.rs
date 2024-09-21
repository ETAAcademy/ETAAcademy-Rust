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


use std:: future::Future;

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