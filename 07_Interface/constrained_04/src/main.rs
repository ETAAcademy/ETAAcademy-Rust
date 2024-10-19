// 1 pub(crate)

// pub mod outer_mod {
//     pub mod inner_mod {
//         // This function is visible within `outer_mod`
//         pub(in crate::outer_mod) fn outer_mod_visible_fn() {}

//         // This function is visible to the entire crate
//         pub(crate) fn crate_visible_fn() {}

//         // This function is visible within `outer_mod`
//         // super is outer_mod
//         pub(super) fn super_mod_visible_fn() {
//             // This function is visible since we're in the same `mod`
//             inner_mod_visible_fn();
//         }

//         // This function is visible only within `inner_mod`,
//         // which is the same as leaving it private.
//         pub(self) fn inner_mod_visible_fn() {}
//     }

//     pub fn foo() {
//         inner_mod::outer_mod_visible_fn();
//         inner_mod::crate_visible_fn();
//         inner_mod::super_mod_visible_fn();

//         // Error! inner_mod_visible_fn is private
//         // inner_mod::inner_mod_visible_fn();
//     }
// }


// fn bar() {
//     outer_mod::inner_mod::crate_visible_fn();

//     // Error! super_mod_visible_fn is private
//     outer_mod::inner_mod::super_mod_visible_fn();

//     // Error! outer_mod_visible_fn is private
//     outer_mod::inner_mod::outer_mod_visible_fn();

//     outer_mod::foo(); 
// }


// fn main() {
//     bar();
// }


// 2 Hidden Type Dependencies

// fn is_true(u: constrained_04::Unit) -> bool {
//     matches!(u, constrained_04::Unit { field: true })
// }

// fn main() {
//     let u = constrained_04::Unit;
// }


// 3 #[non_exhaustive] 

// use constrained_04::Config;
// fn main() {
//     // Error! 
//     let config = Config {
//         window_width: 640,
//         window_height: 480,
//     };

//     if let Config { 
//         window_width, 
//         window_height,
//     } = config {
//         // ....
//     }

// }

// 4 Trait Implementations

// case1
// use constrained_04::{Foo1, Unit};

// case2
// use constrained_04::*;

// // case1 & case2

// trait Foo2 {
//     fn foo(&self);
// }

// impl Foo2 for Unit {
//     fn foo(&self) {
//         println!("foo2 is called");
//     }
// }

// fn main() {
//    // Error: Conflicting `foo` implementations from Foo1 and Foo2.
//     Unit.foo();

// }


// 5 Sealed Trait
use std::fmt::{Debug, Display};
use constrained_04::CanUseCannotImplement;

pub struct Bar {

}

impl Debug for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

// Error! Since CanUseCannotImplement has already been implemented in lib.rs,
impl CanUseCannotImplement for Bar {
    
}

pub struct Foo {}

// Error! Foo does not implement Debug and Display, so it cannot implement CanUseCannotImplement.
impl CanUseCannotImplement for Foo {
    
}

// fn main() {

// }

// 6 Re-exports


fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}

enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
