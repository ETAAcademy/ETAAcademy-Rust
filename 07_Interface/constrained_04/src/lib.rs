// 2 Hidden Type Dependencies

// pub struct Unit {
//     field: bool,
// }


// 3 #[non_exhaustive] 

// #[non_exhaustive]
// pub struct Config {
//     pub window_width: u16,
//     pub window_height: u16,
// }


// fn some_function() {
//     let config = Config {
//         window_width: 640,
//         window_height: 480,
//     };

//     if let Config { 
//         window_width, 
//         window_height
//     } = config {
//         // ....
//     }

// }


// 4 Trait Implementations

// pub struct Unit;

// pub trait Foo1 {
//     fn foo(&self);
// }

// // // case1: add impl Foo1 for Unit in this crate
// // impl Foo1 for Unit {
// //     fn foo(&self) {
// //         println!("foo1 is called");
// //     }
// // }

// // case2: add a new public trait
// pub trait Bar1 {
//     fn foo(&self);
// }

// impl Bar1 for Unit {
//     fn foo(&self) {
//         println!("bar1");
//     }
// }

// 5 Sealed Trait
// use std::fmt::{Debug, Display};

// mod sealed {
//     use std::fmt::{Debug, Display};

//     pub trait Sealed {}
//     impl<T> Sealed for T where T: Debug + Display {
        
//     }
// }

// pub trait CanUseCannotImplement: sealed::Sealed {
//     // ...
// }

// impl<T> CanUseCannotImplement for T where T: Debug + Display {
    
// }


// 6 Re-exports
// pub fn iter<T>() -> itercrate::Empty<T> { 
//     //...
// }

// struct EmptyIterator { it: itercrate::Empty<()>}

// EmptyIterator { it: bestiter:: iter() }

// pub fn iter<T>() -> itercrate::Empty<T> { 
// struct EmptyIterator { it: itercrate::Empty<()>}


// 7 Auto Traits

fn is_normal<T>() where T: Sized + Send + Sync + Unpin {

}

#[test]
fn normal_types() {

    is_normal::<MyType>();
}

