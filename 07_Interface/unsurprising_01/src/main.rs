// ## 1 Debug
use std::fmt::Debug;

use std::fmt::Debug;

#[derive(Debug)]
struct Pair<T> {
    a: T,
    b: T,
}

struct Person {
    name: String,
}

fn main() {
    let pair = Pair { a: 5, b: 10 };
    println!("Pair: {:?}", pair); // This will work because i32 implements Debug

    let pair = Pair {
        a: Person { name: "Dave".to_string() },
        b: Person { name: "Tom".to_string() },
    };
    // This will fail because Person does not implement Debug
    println!("Pair: {:?}", pair);
}

// # 1.2 Debug
// use std::fmt;

// // Define a struct Pair that holds two values of the same type.
// struct Pair<T> {
//     a: T,
//     b: T,
// }

// // Implement the Debug trait for Pair<T> where T implements fmt::Debug.
// impl<T: fmt::Debug> fmt::Debug for Pair<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // Since Pair is a struct, we use the debug_struct method to implement it.
//         f.debug_struct("Pair")    // The name is "Pair".
//             .field("a", &self.a)  // Add the field "a" with its value.
//             .field("b", &self.b)  // Add the field "b" with its value.
//             .finish()             // Finish formatting.
//     }
// }

// fn main() {
//     // Create an instance of Pair with values 5 and 10.
//     let pair = Pair { a: 5, b: 10 };

//     // Print the Pair instance using the Debug trait.
//     println!("Pair: {:?}", pair);
// }

// ## 2 Send 

// use std::rc::Rc;

// #[derive(Debug)]
// struct MyBox(*mut u8);

// unsafe impl Send for MyBox {}

// fn main() {
//     let mb = MyBox(Box::into_raw(Box::new(42)));
//     let x = Rc::new(42);

//     std::thread::spawn(move || {
//         // This will cause an error because Rc does not implement Send
//         println!("{:?}", x);

//         // MyBox implements Send, so this is fine
//         println!("{:?}", mb);
//     });
// }


// ## 2.2 Sync 

// use std::cell::RefCell;
// use std::env::consts::ARCH;
// use std::sync::Arc;

// fn main() {
//     let x = Arc::new(RefCell::new(42));
//     std::thread::spawn(move || {
//         let mut x = x.borrow_mut();
//         // Since RefCell does not implement Sync, the following line causes an error
//         *x += 1; // error: `RefCell<i32>` cannot be shared between threads safely
//     });
// }

// ## 3 Clone 

// #[derive(Debug, Clone)]
// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {
//     fn new(name: String, age: u32) -> Person {
//         Person { name, age }
//     }
// }

// fn main() {
//     let p1 = Person::new("Alice".to_owned(), 22);
//     let p2 = p1.clone();

//     println!("p1: {:?}", p1);
//     println!("p2: {:?}", p2);
// }


// ## 3.2 Default

// #[derive(Default)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point = Point::default();
//     println!("Point: ({}, {})", point.x, point.y);
// }


// ## 4 PartialEq


// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p1 = Point { x: 1, y: 2 };
//     let p2 = Point { x: 1, y: 2 };
//     let p3 = Point { x: 3, y: 4 };

//     println!("Point1 == Point2: {}", p1 == p2);
//     println!("Point1 == Point3: {}", p1 == p3);
// }


// ## 4.2 PartialOrd 

// use std::collections::BTreeMap;

// // Implementing these traits
// // Ord requires PartialOrd
// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Person {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let mut ages = BTreeMap::new();

//     let person1 = Person {
//         name: "Alice".to_owned(),
//         age: 25,
//     };

//     let person2 = Person {
//         name: "Bob".to_owned(),
//         age: 23,
//     };

//     let person3 = Person {
//         name: "Cook".to_owned(),
//         age: 31,
//     };

//     // This will cause an error if PartialOrd is not implemented
//     ages.insert(person1.clone(), "Alice's age");
//     ages.insert(person2.clone(), "Bob's age");
//     ages.insert(person3.clone(), "Cook's age");

//     for (person, desc) in &ages {
//         println!("{}: {} - {:?}", person.name, person.age, desc);
//     }
// }


// ## 4.3 Hash

// use std::collections::HashSet;
// use std::hash::{Hash, Hasher};

// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Person {
//     name: String,
//     age: u32,
// }

// impl Hash for Person {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//         self.age.hash(state);
//     }
// }

// fn main() {
//     let mut persons = HashSet::new();

//     let person1 = Person {
//         name: "Alice".to_owned(),
//         age: 30,
//     };

//     let person2 = Person {
//         name: "Bob".to_owned(),
//         age: 20,
//     };

//     let person3 = Person {
//         name: "Charlie".to_owned(),
//         age: 40,
//     };

//     persons.insert(person1.clone());
//     persons.insert(person2.clone());
//     persons.insert(person3.clone());

//     println!("Person Set {:?}", persons);
// }


// ## 4.4 copy 

// #[derive(Debug, Copy, Clone)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p1 = Point { x: 10, y: 20 };
//     let p2 = p1; // this is a Copy, not a Move.

//     println!("p1: {:?}", p1);
//     println!("p2: {:?}", p2);
// }


// ## 5 Wrapper Types

// use std::ops::Deref;

// // Define a MyVec struct that contains a Vec<i32>.
// struct MyVec(Vec<i32>);

// // Implement the Deref trait for MyVec, targeting Vec<i32>.
// impl Deref for MyVec {
//     type Target = Vec<i32>;

//     // The deref method returns a reference to the inner Vec<i32>.
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     // Create an instance of MyVec containing a vector of integers.
//     let my_vec = MyVec(vec![1, 2, 3, 4, 5]);

//     //  Since MyVec implements Deref, we can directly call the len() method of Vec<i32> on an instance of MyVec, and also access the first element using indexing
//     println!("Length: {}", my_vec.len());
//     println!("First element: {}", my_vec[0]);
// }

// # 5.2 AsRef

// use std::ops::Deref;

// #[derive(Debug)]
// struct Wrapper(String);

// impl Deref for Wrapper {
//     type Target = String;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl AsRef<str> for Wrapper {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }

// impl From<String> for Wrapper {
//     fn from(s: String) -> Self {
//         Wrapper(s)
//     }
// }

// impl From<Wrapper> for String {
//     fn from(wrapper: Wrapper) -> Self {
//         wrapper.0
//     }
// }

// fn main() {
//     // Create a Wrapper instance from a String.
//     let wrapper = Wrapper::from("Hello".to_string());

//     // Use the . operator to call a method on the inner String.
//     println!("Length: {}", wrapper.len());

//     // Use the as_ref method to convert Wrapper to a &str type.
//     let inner_ref: &str = wrapper.as_ref();
//     println!("Inner: {}", inner_ref);

//     // Convert Wrapper to its inner String type.
//     let inner_string: String = wrapper.into();
//     println!("Inner String: {}", inner_string);

//     // Uncommenting the following line will convert a String directly into a Wrapper.
//     // let w2: Wrapper = "World".to_string().into();
//     // println!("w2 Wrapper: {:?}", w2);
// }


// 16 borrow

// use std::borrow::Borrow;

// fn print_length<S>(string: S) where S: Borrow<str>, {
//     println!("Length: {}", string.borrow().len());
// }

// fn main() {
//     let str1: &str = "Hello";
//     let string1: String = String::from("World");

//     print_length(str1);
//     print_length(string1);
// }
