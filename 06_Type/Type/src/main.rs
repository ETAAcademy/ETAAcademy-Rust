static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {

    let a = 42;
    let b = &B;
    let c = &C;

    /*
        {:p} prints the pointer’s address
    */
    println!("a: {}, b: {:p}, c: {:p}", a, b, c); // a: 42, b: 0x10c7ea44c, c: 0x10c7ea456
}


use std::mem::size_of;
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: usize = 42;
    let b: Box<[u8]> = Box::new(B); // Ownership of B moves to Box
    let c: &[u8; 11] = &C;

    println!("a (unsigned integer):");
    println!("  Address: {:p}", &a);
    println!("  Size: {:?} bytes", size_of::<usize>());
    println!("  Value:   {:?}\n", a);

    println!("b (B inside Box):");
    println!("  Address: {:p}", &b);
    println!("  Size: {:?} bytes", size_of::<Box<[u8]>>());
    println!("  Points to: {:p}\n", b);

    println!("c (Reference to C):");
    println!("  Address: {:p}", &c);
    println!("  Size: {:?} bytes", size_of::<&[u8; 11]>());
    println!("  Points to: {:p}\n", c);

    println!("B (10 bytes array):");
    println!("  Address: {:p}", &B);
    println!("  Size: {:?} bytes", size_of::<&[u8; 10]>());
    println!("  Value: {:?}\n", B);

    println!("C (11 bytes array):");
    println!("  Address: {:p}", &C);
    println!("  Size: {:?} bytes", size_of::<&[u8; 11]>());
    println!("  Value: {:?}\n", C);
}




// Cow is a smart pointer that represents "copy on write".
// It only performs a copy when writing is needed. During reads, it doesn't copy.
use std::borrow::Cow;
// CStr is similar to C strings, allowing us to read null-terminated strings.
use std::ffi::CStr;
// c_char is a Rust alias for `i8`.
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a = 42;
    let b: String;    // Smart pointer
    let c: Cow<str>;  // Smart pointer

    unsafe {
        // Executing within an unsafe block.
        // *mut u8 is a mutable raw pointer.
        let b_ptr = &B as *const u8 as *mut u8;  // Raw pointer
        b = String::from_raw_parts(b_ptr, 10, 10);

        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("a: {} b: {} c: {}", a, b, c);  // a: 42 b: carrytowel c: thanksfish
}



fn main() {
    let a: i64 = 42;
    // *const i64, turning the reference `&a` into a *const i64
    let a_ptr = &a as *const i64;

    println!("a: {}, ({:p})", a, a_ptr); // a: 42, (0x7ff7b358c1b0)
}



fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;

    let a_addr: usize = unsafe {
        // Using transmute to convert into a usize type
        std::mem::transmute(a_ptr)
    };
    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7); // a: 42 (0x7ff7bf91f170...0x7ff7bf91f177)
}


fn main() {
    // Four values: 42, 43, &x, &y
    // Four variables: x, y, var1, var2
    // var1 and var2 are pointer types (references)
    let x = 42;
    let y = 43;
    let var1 = &x;
    let mut var2 = &x;
    var2 = &y;

    // `s` is a pointer to the first character of the string "Hello World"
    let s = "Hello World";
}



fn main() {
    let pw = "jackdss";
    let is_strong = is_strong(pw);
}

// &str -> Stack; String -> Heap
// This version accepts `String`, but passing in an `&str` would result in an error
// fn is_strong(password: String) -> bool {
//     password.len() > 5
// }

// This version works with types that implement `AsRef<str>` (such as `&str` or `String`)
fn is_strong<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}

// This version works with types that can be converted into `String`
fn is_strong<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}



fn main() {
    let a: i32 = 40;          // Stored on the Stack
    let b: Box<i32> = Box::new(30);  // Stored on the Heap

    // Error: `b` is stored on the Heap, so we must dereference it to access the value
    // let result = a + b;

    // Correct usage:
    let result = a + *b;

    println!("{} + {} = {}", a, b, result); // Output: 40 + 30 = 70
}



// Static global variable
static GLOBAL: i32 = 1000;

fn noop() -> *const i32 {
    let noop_local = 123456;
    &noop_local as *const i32
}

fn main() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    println!("GLOBAL:        {:p}", &GLOBAL as *const i32);    // GLOBAL:        0x10df4b190
    println!("local_str:     {:p}", local_str as *const str);  // local_str:     0x10df4b194
    println!("local_int:     {:p}", &local_int as *const i32); // local_int:     0x7ff7b1ff3f8c
    println!("boxed_int:     {:p}", Box::into_raw(boxed_int)); // boxed_int:     0x7fefeef05b50
    println!("boxed_str:     {:p}", Box::into_raw(boxed_str)); // boxed_str:     0x7fefeef05b40
    println!("fn_int:        {:p}", fn_int);                   // fn_int:        0x7ff7b1ff3ebc
}


fn main() {
    let mut n_nonzero = 0;

    // Scanning memory starting from address 0
    // i == 0 is a null pointer, which is illegal
    for i in 0..10000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("Non-zero bytes in memory: {}", n_nonzero);  // Error: segmentation fault
}

fn main() {
    let x1 = 42;        // x1 implements Copy, so it's copied.
    let y1 = Box::new(88);  // y1 is allocated on the heap.

    {
        // y1 is moved into z, while x1 is copied.
        let z = (x1, y1);
    }   // z and its contents (including y1) are dropped here.

    let x2 = x1;        // x1 can still be used after the move.

    let y2 = y1;        // Error: y1 was moved and cannot be used again.

}



// input and output point to different memory locations because output is mutable and exclusive.
// Modifying output doesn't affect input.
fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
}



fn main() {
    let x = 42;
    let mut y = &x;
    let z = &mut y;

    // You can make y point to another value, but you can't modify x via y
    // because z is not mutable, it only holds a mutable reference to y.

    // *y = 10;  // Error

    let n = 20;
    *z = &n;  // Modifying y to point to &n
}


fn main() {
    let mut s = Box::new(42);
    replace_with_84(&mut s);
}

fn replace_with_84(s: &mut Box<i32>) {
    // Moving *s would leave it empty, which is not allowed because the caller still believes it owns the value.
    // let was = *s;

    // Instead, use std::mem::take to move the value out and replace it with a default.
    let was = std::mem::take(s);
    println!("was init is {}", was);  // 42
    println!("was init s is {}", s);  // 0

    // Re-assign the original value back to s
    *s = was;
    println!("*s = was is {}", s);    // 42

    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}


fn main() {
    let mut x = Box::new(42);
    let r = &x;    // 'a lifetime begins here.

    if rand::random::<f32>() > 0.5 {
        *x = 84;   // Mutable borrow of x is valid here.
    } else {
        println!("{}", r);  // r is still valid because the borrow checker knows the code path.
    }
}

fn main() {
    let mut x = Box::new(42);
    let mut z = &x;  // 'a lifetime starts here.

    for _ in 0..100 {
        println!("{}", z);
        x = Box::new(1);    // Previous lifetime ends.
        z = &x;             // New lifetime starts.
    }

    println!("{}", z);
}

use std::path::Iter;

struct StrSplit<'s, 'p> {
    delimiter: &'p str,   // Delimiter
    document: &'s str,    // Document
}

// Implementing the Iterator trait
impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}

struct StrSplit<'s> {
    delimiter: &'s str,   // Delimiter
    document: &'s str,    // Document
}

// Implementing the Iterator trait
impl<'s> Iterator for StrSplit<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}

// Example of covariance
let x1: &'static str;   // Longer lifetime, lives as long as the program
let x2: &'a str;        // Shorter lifetime

// Functions
fn take_func1(&'static str) { /* more restrictive */ }
fn take_func2(&'s str) { /* more lenient */ }

struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

fn main() {
    let mut r: &str = "hello";  // &'static str -> &'a str

    *MutStr { s: &mut r }.s = "world";  // Using multiple lifetimes
    println!("{}", r);  // Prints "world", because 'b's lifetime allows for this modification
}

impl String {
    pub fn contains(&self, p: impl Pattern) -> bool {
        p.is_contained_in(self)
    }
}

impl String {
    pub fn contains(&self, p: &dyn Pattern) -> bool {
        p.is_contained_in(&*self)
    }
}

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}


struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;

}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, num_1: &i32, num_2: &i32) -> bool {
        (&self.0 == num_1) && (&self.1 == num_2)
    }

    fn first(&self) -> i32 { self.0 }

    fn last(&self) -> i32 { self.1 }
}


pub trait Deref {
		type Target: ?Sized;

		fn deref(&self) -> &Self::Target;
}



pub trait Iterator {
		type Item;
}


// Allowed implementations
impl<T> From<T> for MyType;
impl<T> From<MyType> for Vec<T>;
impl<T> ForeignTrait<MyType, T> for Vec<T>;

// Not allowed
impl<T> ForeignTrait for T;
impl<T> From<T> for T;
