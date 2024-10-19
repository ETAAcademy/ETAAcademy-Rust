// # 1 A Looser Contract Using `Cow`
use std::borrow::Cow;

// A looser contract
fn frobnicate3<T: AsRef<str>>(s: T) -> T {
    s
}

fn main() {
    let string = String::from("example");
    let borrowed: &str = "hello";
    let cow: Cow<str> = Cow::Borrowed("world");

    let result1: &str = frobnicate3::<&str>(string.as_ref());
    let result2: &str = frobnicate3::<&str>(borrowed);
    let result3: Cow<str> = frobnicate3(cow);

    println!("Result1: {:?}", result1);
    println!("Result2: {:?}", result2);
    println!("Result3: {:?}", result3);
}

// 2 Using Generics
// There is a function that takes a parameter of type AsRef<str>
// fn print_as_str<T>(s: T) where T: AsRef<str> {
//     println!("{}", s.as_ref());
// }

// This function is generic; it is parameterized over T.
// This means it will monomorphize for each type that implements AsRef<str> that you use it with.
// Solution: Change it to accept &dyn AsRef<str>
// This means it will use dynamic dispatch at runtime to call the as_ref method, and you will only have one copy of the function in your binary.

// fn print_as_str2(s: &dyn AsRef<str>) {
//     println!("{}", s.as_ref());
// }

// fn main() {
//     let s = String::from("hello");
//     let r = "world";

//     // print_as_str(s); // Calls print_as_str::<String>
//     // print_as_str(r); // Calls print_as_str::<&str>

//     print_as_str2(&s); // Passes a trait object of type &dyn AsRef<str>
//     print_as_str2(&r); // Passes a trait object of type &dyn AsRef<str>
// }

// 3 Static Dispatch vs Dynamic Dispatch


// Static Dispatch: This kind of function uses static dispatch, which means that implementations for each specific type T will be generated at compile time.
// fn process<T>(value: T) {
//     // Code to process value
//     println!("Processing T");
// }

// Dynamic Dispatch: Implements runtime selection of implementations. They can be achieved by passing trait objects as parameters, using the dyn keyword, here’s an example.
// trait Processable {
//     fn process(&self);
// }

// #[derive(Debug)]
// struct TypeA;
// impl Processable for TypeA {
//     fn process(&self) {
//         println!("Processing TypeA");
//     }
// }

// struct TypeB;
// impl Processable for TypeB {
//     fn process(&self) {
//         println!("Processing TypeB");
//     }
// }


// The received type is a trait object.
// It requires the object to implement Processable.
// If the caller wants to use dynamic dispatch and select the implementation at runtime,
// they can call the process_trait_object function and pass a trait object as a parameter.
// The caller can choose which specific implementation to provide based on their needs (programming to an interface).
// fn process_trait_object(value: &dyn Processable) {
//     value.process();
// }

// fn main() {
//     let a = TypeA;
//     let b = TypeB;

//     // Call using dynamic dispatch
//     process_trait_object(&a);
//     process_trait_object(&b);

//     // Call using static dispatch, passing different types produces different type implementations
//     process(&a);
//     process(&b);
//     process(&a as &dyn Processable);
//     process(&b as &dyn Processable);

//     println!("TypeA {:?}", a);
// }

// 4 Recommendation: Start writing interfaces from `concrete types` and gradually turn them into generics as exceptions

// fn foo(v: &Vec<usize>) {
//     // Code...
// }

// // Change to use a trait bound of AsRef<[usize]> i.e., impl AsRef<[usize]>
// fn foo2(v: impl AsRef<[usize]>) {
//     // Code...
// }

// fn main() {
    
//     let iter = vec![1, 2, 3].into_iter();

//     // Calling foo is fine because the compiler can infer that iter.collect() should collect into a Vec<usize>, since we passed it to foo which accepts &Vec<usize>.
//     // foo(&iter.collect());

//     // To resolve this issue: You need to specify a concrete type, either foo2(&iter.collect::<Vec<usize>>())
//     // or 
//     // let list: Vec<usize> = iter.collect();
//     // foo2(&list)
//     // let list: Vec<usize> = iter.collect();
//     // foo2(&list);
//     foo2(&iter.collect::<Vec<usize>>());
// }

// 5 Example of Object Safety
/*
    Suppose we have an Animal trait that has two methods: name and speak.
    The name method returns a &str that indicates the name of the animal,
    and the speak method prints the sound the animal makes.
    We can implement these traits for Dog and Cat types.
*/

// trait Animal {
//     fn name(&self) -> &str;
//     fn speak(&self);
// }

// struct Dog {
//     name: String
// }

// impl Animal for Dog {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("woof!");
//     }
// }

// struct Cat {
//     name: String
// }

// impl Animal for Cat {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("Meow!");
//     }
// }

/*
    This Animal trait is object-safe because it does not return Self types or use generic parameters.
    Thus, we can create a trait object.

    This allows us to use a unified type Vec<&dyn Animal> to store different types of animals,
    and call their methods through the trait object.
*/

// fn main() {
//     let dog = Dog { name: "Fido".to_string() };

//     let cat = Cat { name: "Whiskers".to_string() };

//     let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    
//     for animal in animals {
//         println!("This is {}", animal.name());
//         animal.speak();
//     }
// }

// 6 Non-object Safe Example

// trait Animal {
//     fn name(&self) -> &str;
//     fn speak(&self);
//     fn clone(&self) -> Self; 
// }

// struct Dog {
//     name: String,
// }

// impl Animal for Dog {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("wang wang!");
//     }

//     fn clone(&self) -> Self where Self: Sized, {
//         todo!()
//     }
// }

// struct Cat {
//     name: String,
// }

// impl Animal for Cat {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("miao miao!");
//     }

//     fn clone(&self) -> Self where Self: Sized, {
//         todo!()
//     }
// }

// fn main() {
//     let dog = Dog { name: "Fido".to_string() };

//     let cat = Cat { name: "Whiskers".to_string() };

//     let animals: Vec<&dyn Animal> = vec![&dog, &cat];

//     for animal in animals {
//         println!("This is {}", animal.name());
//     }
// }


// 6.2 Change to keep object safety

// trait Animal {
//     fn name(&self) -> &str;
//     fn speak(&self);
//     fn clone(&self) -> Self where Self: Sized; 
// }

// struct Dog {
//     name: String,
// }

// impl Animal for Dog {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("woof woof!");
//     }

//     fn clone(&self) -> Self where Self: Sized, {
//         todo!()
//     }
// }


// #[derive(Debug)]
// struct Cat {
//     name: String,
// }

// impl Animal for Cat {
//     fn name(&self) -> &str {
//         &self.name
//     }

//     fn speak(&self) {
//         println!("meow meow!");
//     }

//     fn clone(&self) -> Self where Self: Sized, {
//         Cat {
//             name: self.name.clone(),
//         }
//     }
// }

// fn main() {
//     let dog = Dog { name: "Fido".to_string() };

//     let cat = Cat { name: "Whiskers".to_string() };


//         This way, we can continue to create trait objects with Animal
//     let animals: Vec<&dyn Animal> = vec![&dog, &cat];

//     for animal in animals {
//         println!("This is {}", animal.name());
//         animal.speak();
//         /*
//             Note: However, we cannot call the clone method on trait objects
//             because the clone method can only be called on concrete types
//          */
//         //animal.clone();  // error
//     }

//     // Because we can only call the clone method on concrete types
//     let cat2: Cat = cat.clone();
//     println!("Cloned cat is {:?}", cat2);
// }
    

// 7 Putting generic parameters on the trait

// use std::collections::HashSet;
// use std::hash::Hash;

// // Put the generic parameter on the Trait
// trait Container<T> {
//     fn contains(&self, item: &T) -> bool;
// }

// // We can implement the Container Trait for different container types, 
// // each implementation has its specific element type
// // For example, we can implement the Container Trait for Vec<T> and HashSet<T>
// impl<T> Container<T> for Vec<T> 
//     where T: PartialEq, 
// {
//         fn contains(&self, item: &T) -> bool {
//             self.iter().any(|x| x == item)
//         }
// }

// impl<T> Container<T> for HashSet<T> 
// where T: Hash + Eq, 
// {   
//     fn contains(&self, item: &T) -> bool {
//         self.contains(item)
//     }
// }

// fn main() {
//     let vec_container: Box<dyn Container<i32>> = Box::new(vec![1, 2, 3]);

//     let set_container: Box<dyn Container<i32>> = Box::new(
//         vec![4, 5, 6].into_iter().collect::<HashSet<_>>()
//     );

//     // Call the contains method

//     println!("Vec contains 2: {}", vec_container.contains(&2));
//     println!("HashSet contains 6: {}", set_container.contains(&6));
// }


// 8 Can generic parameters use dynamic dispatch to ensure trait object safety

// use std::fmt::Debug;

// /*
//     There is a trait Foo with a generic method bar that takes a generic parameter T:
//     trait Foo {
//         fn bar<T>(&self, x: T);
//     }
//     Is this Foo object-safe? The answer is: it depends on the type of T
//     1 If T is a concrete type like i32 or String, it is `not` object-safe,
//     because it needs to know the specific type of T at runtime to call the bar method

//     2 If T is a trait object like &dyn Debug or &dyn Display,
//     then this trait is object-safe because it can dynamically dispatch 
//     the method of T, so we define it as follows:
// */

// trait Foo {
//     fn bar(&self, x: &dyn Debug);
// }

// // Define A to implement Foo
// struct A {
//     name: String,
// }

// impl Foo for A {
//     fn bar(&self, x: &dyn Debug) {
//         println!("A {} says {:?}", self.name, x);
//     }
// }

// // Define B to implement Foo
// struct B {
//     id: i32,
// }

// impl Foo for B {
//     fn bar(&self, x: &dyn Debug) {
//         println!("B {} says {:?}", self.id, x);
//     }
// }

// fn main() {
//     // Now we can create trait objects using Foo

//     let a = A { name: "Bob".to_string() };

//     let b = B { id: 42 };

//     // Create a Vec that stores Foo trait objects
//     let foos: Vec<&dyn Foo> = vec![&a, &b];

//     // Iterate over the Vec and call the bar method on the trait object
//     for foo in foos {
//         // & makes &str => &dyn Debug
//         foo.bar(&"Hello"); // "Hello" implements the Debug trait
//     }
    
// }


// 9 It is common to use `Cow` in return types to indicate that a function 
// may sometimes allocate memory

// use std::borrow::Cow;
// // There is a function process_data that takes a string parameter. Sometimes we need to modify the input string and take ownership of the modified string. However, most of the time, we only need to read the input string without modifying it
// fn process_data(data: Cow<str>) {
//     if data.contains("invalid") {
//         // If the input string contains "invalid", we need to modify it
//         // into_owned takes ownership and returns the owned data
//         let owned_data: String = data.into_owned();
//         // Perform some modification
//         println!("Processed data: {}", owned_data);
//     } else {
//         // If the input string does not contain "invalid", we just read it
//         println!("Data: {}", data);
//     }
// }

// // When calling, we can pass either a regular string reference (&str) or an owned string (String) as the parameter

// fn main() {
//     let input1 = "This is valid data.";
//     process_data(Cow::Borrowed(input1));       // Pass a reference

//     let input2 = "This is invalid data.";
//     process_data(Cow::Owned(input2.to_owned())); // Pass owned data
// }


// 10 Destructor

// use std::os::fd::AsRawFd;

// struct File {
//     name: String,
//     fd: i32,
// }

// impl File {
//     fn open(name: &str) -> Result<File, std::io::Error> {
//         // Use std::fs::OpenOptions to open a file with read and write permissions
//         let file = std::fs::OpenOptions::new()
//             .read(true)
//             .write(true)
//             .open(name)?;

//         // Use std::os::unix::io::AsRawFd to get the file descriptor
//         let fd = file.as_raw_fd();
        
//         // Return a File instance containing the name and fd fields
//         Ok(File { 
//             name: name.to_string(),
//              fd, 
//         })
//     }

//     // A visible destructor function that closes the file and returns an error if any
//     fn close(self) -> Result<(), std::io::Error> {
//         // use std::os::unix::io::FromRawFd to convert fd back to std::fs::File
//         let file: std::fs::File = unsafe {
//             std::os::unix::io::FromRawFd::from_raw_fd(
//                 self.fd
//             )
//         };
//         // use std::fs::File::sync_all to flush any pending writes to disk
//         file.sync_all()?;
//         // use std::fs::File::set_len to truncate the file to 0 bytes
//         file.set_len(0)?;
//         // use std::fs::File::sync_all again to flush the truncation
//         file.sync_all()?;

//         // Drop the file instance; it will be closed automatically
//         drop(file);

//         // Return Ok(())
//         Ok(())
//     }
// }

// fn main() {
//     // Create a file named "test.txt" containing some content
//     std::fs::write("test.txt", "Hello, world!").unwrap();

//     // Open the file and get a File instance
//     let file = File::open("test.txt").unwrap();

//     println!("File name: {}, fd: {}", file.name, file.fd);

//     // Close the file and handle any errors
//     // Destructor close is called
//     match file.close() {
//         Ok(_) => println!("File closed successfully"),
//         Err(e) => println!("Error closing file: {}", e),
//     }

//     // Check the size of the file after closing
//     let metadata = std::fs::metadata("test.txt").unwrap();
//     println!("File size: {} bytes", metadata.len());
// }


// 11 Drop does not own self

// use std::os::fd::AsRawFd;

// struct File {
//     name: String,
//     fd: i32,
// }

// impl File {
//     fn open(name: &str) -> Result<File, std::io::Error> {
//         // Use std::fs::OpenOptions to
