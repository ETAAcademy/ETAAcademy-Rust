use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello before reading file!");
    let file1_contents = read_from_file1();

    println!("{:?}", file1_contents);
    println!("Hello after reading file1!");
    let file2_contents = read_from_file2();
    println!("{:?}", file2_contents);
    println!("Hello after reading file2!");
}

fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    String::from("Hello, there from file 1")
}

fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    String::from("Hello, there from file 2")
}