# ETAAcademy-Rust: 06. Type & Pointer

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>06. Type & Pointer</td>
    <td>
      <table>
        <tr>
          <th>rust</th>
          <th>basic</th>
          <td>Type_Pointer</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https:github.com/ETAAcademy)｜[Twitter](https:twitter.com/ETAAcademy)｜[ETA-ZK-Rust](https://github.com/ETAAcademy/ETAAcademy-Rust)

Authors: [Eta](https:twitter.com/pwhattie), looking forward to your joining

## 1. Pointers and Memory Addresses

Rust is a systems programming language that provides powerful tools for memory management and efficient handling of low-level details like pointers and memory addresses. While Rust emphasizes safety, it also allows direct memory manipulation through pointers, raw pointers, and references.

### What Is a Pointer?

A pointer is a memory address that holds the location of data in memory rather than the data itself. In Rust, a reference (`&T` or `&mut T`) is a type of pointer, which the compiler ensures is valid, preventing issues like dangling pointers or memory corruption. When using a pointer, you interact with memory by dereferencing it to access the actual data it points to.

In simple terms, pointers are like a signpost that directs you to the location where data is stored. In high-level languages, references are abstracted pointers with built-in safety mechanisms. However, Rust also allows you to use raw pointers for scenarios where safety checks may be too restrictive.

In Rust, references are the most common form of pointers. A reference points to a value in memory and can be either mutable or immutable. References are memory-safe; they prevent issues such as dangling references and double borrowing of mutable data. To access the data stored at the location a pointer refers to, we use dereferencing. In Rust, you can dereference both raw pointers and smart pointers to extract the value they point to.

<details><summary>Code</summary>

```rust
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

```

```rust
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

```

Output example:

```rust
a (unsigned integer):
  Address: 0x7ff7b73eeb48
  Size: 8 bytes
  Value:   42

b (B inside Box):
  Address: 0x7ff7b73eeb50
  Size: 16 bytes
  Points to: 0x7fcd17f05b20 // The address is on the heap. Since B is a static variable, it resides in the static memory area. However, when placed in a Box, a new address is generated in the heap.

c (Reference to C):
  Address: 0x7ff7b73eeb70
  Size: 8 bytes
  Points to: 0x108b5009e    // C is essentially a raw pointer (reference), and this points to the address of C below.

B (10 bytes array):
  Address: 0x108b50094
  Size: 8 bytes
  Value: [99, 97, 114, 114, 121, 116, 111, 119, 101, 108]

C (11 bytes array):
  Address: 0x108b5009e
  Size: 8 bytes
  Value: [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0]

```

</details>

### Raw Pointers: Power with Caution

Raw pointers in Rust are a more dangerous but flexible type of pointer that does not have the safety guarantees provided by references. They are similar to pointers in C/C++ and allow more direct interaction with memory. However, raw pointers are "unsafe," meaning you must explicitly declare when you're using them and handle them carefully.

Rust provides two types of raw pointers:

- `*const T` for immutable raw pointers
- `*mut T` for mutable raw pointers

Unlike references, raw pointers can be freely cast between mutable and immutable forms, but the compiler won't enforce memory safety rules for them.

<details><summary>Code</summary>

```rust

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

```

```rust

fn main() {
    let a: i64 = 42;
    // *const i64, turning the reference `&a` into a *const i64
    let a_ptr = &a as *const i64;

    println!("a: {}, ({:p})", a, a_ptr); // a: 42, (0x7ff7b358c1b0)
}

```

```rust

fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;

    let a_addr: usize = unsafe {
        // Using transmute to convert into a usize type
        std::mem::transmute(a_ptr)
    };
    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7); // a: 42 (0x7ff7bf91f170...0x7ff7bf91f177)
}

```

</details>

### Smart Pointers

Rust introduces **smart pointers**, which extend the basic concept of raw pointers by adding extra functionality like automatic memory management. Smart pointers are commonly used in data structures and are central to how Rust manages ownership, borrowing, and lifetimes.

Some of the most popular smart pointers in Rust include:

- **`Box<T>`**: A heap-allocated pointer.
- **`Rc<T>`**: A reference-counted smart pointer that allows multiple ownership of data.
- **`Arc<T>`**: An atomic version of `Rc<T>`, safe to use in multi-threaded environments.
- **`RefCell<T>`** and **`Cell<T>`**: Smart pointers that allow interior mutability, enabling you to mutate data even if it's behind an immutable reference.

# 2. RAM

Memory can be stored and accessed in different ways, depending on the type and usage of variables and pointers.

### Values and Variables: The Building Blocks of Memory

At the core of any program, **values** and **variables** are fundamental concepts. A **value** in Rust consists of a type and an element from the value's domain, which can be represented as a sequence of bytes. However, it's important to distinguish between a value itself and the memory location where it’s stored. For example, the number `6` in Rust is an integer of type `u8`, but in memory, it's represented as `0x06`.

A **variable**, on the other hand, serves as a named location where values are stored. Variables can be stored in various memory regions like the **Stack** or **Heap**. They act as "slots" in which values reside, and their primary function is to hold and reference these values during a program’s execution.

In Rust, a **pointer** is a value that contains a memory address. This address points to the location where a certain value is stored. Pointers allow us to access the values in memory indirectly. By dereferencing the pointer, we can retrieve the value stored at that specific memory location. Moreover, pointers can be stored in multiple variables, allowing different variables to reference the same underlying value in memory.

<details><summary>Code</summary>

```rust
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

```

</details>

### High-Level vs. Low-Level Models of Variables

In Rust, variables can be understood from two perspectives:

1. **High-Level Model**: This perspective focuses on Rust’s safety mechanisms, such as lifetimes and borrowing, where variables represent a named value.
2. **Low-Level Model**: From a more granular viewpoint, variables are "slots" that store values. When you assign a value to a variable, it fills the slot, and any previous value is discarded. If a variable doesn’t require a memory address, the compiler may store it in a register rather than in memory.

**Memory Regions in Rust: Stack, Heap, and Static Memory**

Rust uses several memory regions to store variables and manage data. These include the **Stack**, **Heap**, and **Static Memory**.

1. **Stack Memory**: The Stack is where variables of known size are stored. When functions are called, memory is allocated on the Stack in the form of **stack frames**. As more functions are called, the Stack grows, and the **stack pointer** moves toward the bottom of the Stack. When functions return, the stack pointer moves upward again, deallocating memory as necessary.

<details><summary>Code</summary>

```rust

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

```

</details>

2. **Heap Memory**: Unlike the Stack, which is bound to function calls, the **Heap** is a memory pool designed for storing data whose size is not known at compile-time. This is typically where dynamically-sized types, such as `String` and `Vec<T>`, are stored. Since heap-allocated values are not tied to the program’s call stack, they must be accessed via pointers.

<details><summary>Code</summary>

```rust

fn main() {
    let a: i32 = 40;          // Stored on the Stack
    let b: Box<i32> = Box::new(30);  // Stored on the Heap

    // Error: `b` is stored on the Heap, so we must dereference it to access the value
    // let result = a + b;

    // Correct usage:
    let result = a + *b;

    println!("{} + {} = {}", a, b, result); // Output: 40 + 30 = 70
}

```

</details>

3. **Static Memory**: **Static memory** refers to memory regions that exist for the entire duration of a program. Variables with a `'static` lifetime, such as those declared with the `static` keyword, reside in this memory region. These variables are automatically loaded into memory when the program starts and remain there until the program ends.

<details><summary>Code</summary>

```rust

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

```

</details>

### Dynamic Memory Allocation

As programs run, they may need additional memory, which is allocated dynamically. **Dynamic memory allocation** involves requesting memory from the operating system at runtime. Rust achieves this using pointers and types like `Box`, `Rc`, and `Arc`, which manage heap memory for dynamically sized types.

**Virtual Memory**

Modern systems use **virtual memory** to manage large datasets and complex programs. Virtual memory gives each program the illusion of having its own contiguous block of memory, even though the actual data may be scattered across physical memory. Virtual addresses are divided into blocks called **pages**, typically 4KB in size, which helps optimize memory usage and reduces the overhead of managing physical memory directly.

<details><summary>Code</summary>

```rust
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
```

</details>

# 3.Ownership and Traits

In Rust, ownership forms the backbone of its memory management model. At the heart of this system is the principle that every value in Rust has a single owner at any given time. This ownership system enables Rust to handle memory efficiently without the need for a garbage collector, ensuring safety and preventing issues like data races and null pointer dereferencing.

#### Ownership: The Core Concept

Ownership in Rust dictates that each value has one and only one owner, which serves as the value's "location" in memory. When a variable goes out of scope, its associated value is automatically dropped, meaning the memory is freed and the value is discarded. Rust recursively applies this drop mechanism to any values contained within a type, ensuring efficient memory management.

However, when references to other values are involved, those values are not dropped when the variable is destroyed. Instead, the value's ownership can be transferred, or "moved," to a new location, such as when a variable is reassigned or when a value is pushed into a `Vec` or placed on the heap. For types that implement the `Copy` trait (like primitive types such as integers and floats), assigning a value to a new location results in a copy rather than a move.

**Drop Order:**
In Rust, variables are dropped in the reverse order of their declaration. Nested values within complex types are dropped in the order they appear in the code. This ensures predictable and safe resource deallocation.

<details><summary>Code</summary>

```rust
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

```

</details>

### References and Interior Mutability

Rust introduces the concept of references to enable borrowing of values without taking ownership. These references come in two forms: shared (immutable) references (`&T`) and mutable references (`&mut T`).

**Shared References (`&T`)** allow multiple immutable borrows of a value at the same time. These references are `Copy` by default, so you can freely duplicate them without transferring ownership. A key feature of shared references is that the value they reference cannot be changed during the borrow.

On the other hand, **mutable references (`&mut T`)** are exclusive, meaning that if a mutable reference exists, no other references (shared or mutable) can exist simultaneously. This exclusivity ensures that modifications are made safely without the risk of data races or inconsistencies.

<details><summary>Code</summary>

```rust

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

```

Mutable references allow modification of the memory they point to:

```rust

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

```

</details>

**Ownership vs. Mutable References**

A critical distinction between owning a value and borrowing it via a mutable reference is that the owner is responsible for dropping the value when it's no longer needed, while the borrower (via a mutable reference) can modify but not take ownership of the value.

<details><summary>Code</summary>

```rust
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

```

</details>

**Interior Mutability**

Rust provides special types, such as `Mutex` and `RefCell`, that allow **interior mutability**—modifying a value through a shared reference. These types use an underlying `UnsafeCell`, which is the only way to mutate values behind shared references safely.

Other types like `std::sync::atomic` and `std::cell::Cell` allow replacing values atomically or cell-by-cell, but do not allow obtaining direct references to the underlying value, ensuring thread safety and preventing data races.

### Lifetimes: Managing References

In Rust, **lifetimes** define the scope during which a reference is valid. The borrow checker uses lifetimes to ensure that references do not outlive the data they point to. While Rust often infers lifetimes automatically, in some cases, you need to explicitly annotate them.

<details><summary>Code</summary>

```rust
fn main() {
    let mut x = Box::new(42);
    let r = &x;    // 'a lifetime begins here.

    if rand::random::<f32>() > 0.5 {
        *x = 84;   // Mutable borrow of x is valid here.
    } else {
        println!("{}", r);  // r is still valid because the borrow checker knows the code path.
    }
}
```

</details>

In some scenarios, lifetimes must be "restarted" to accommodate new references to the same data:

<details><summary>Code</summary>

```rust
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
```

</details>

**Generic Lifetimes in Rust**

In Rust, lifetimes can be generic over one or more references. You can define types that are generic over multiple lifetimes, which is particularly useful when your type contains multiple references. However, you should only use multiple lifetime parameters if your type contains multiple references, and the reference returned by a method should be tied to one of the input lifetimes.

Consider the following example where two lifetimes, `'s` and `'p`, are used. The code works without any issues:

<details><summary>Code</summary>

```rust
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
```

</details>

In this code, both the delimiter and document have different lifetimes `'s` and `'p`, and Rust can handle this scenario without issues. The delimiter points to a string (`&p str`), while the document has its own separate lifetime (`&s str`).

However, if you try to reduce the code to use a single lifetime, you might encounter problems, especially when temporary variables or values created within the function come into play. This is because the temporary value created within the function will have a short lifetime and cannot be referenced after the function ends.

Here's an example with one lifetime:

<details><summary>Code</summary>

```rust
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
```

</details>

In this case, there's an error because the `&c.to_string()` creates a temporary value inside the `str_before` function. The lifetime of this temporary value is shorter than the input string `s`, causing Rust to throw an error as it cannot reference a value with a shorter lifetime.

**Variance in Lifetimes**

Variance refers to how subtypes and supertypes can be substituted for one another. If type A is a subtype of type B, then A can be used wherever B is expected. For example, if a function expects a parameter of type `&'a str`, you can pass a `&'static str` because `'static` lives as long as any other lifetime `'a`. `'static` means that the value lives for the entire duration of the program, such as string literals.

There are three types of variance:

1. **Covariance**: A type can be replaced with a subtype. For instance, a function that expects a `&'a str` can accept a `&'static str` because `'static` is a longer lifetime.
2. **Invariant**: Requires the exact same type. For example, `&mut T` is invariant, meaning it must receive the precise type, as mutability enforces stricter rules.
3. **Contravariance**: A function's argument requirements can be lowered to accept broader types.

<details><summary>Code</summary>

```rust
// Example of covariance
let x1: &'static str;   // Longer lifetime, lives as long as the program
let x2: &'a str;        // Shorter lifetime

// Functions
fn take_func1(&'static str) { /* more restrictive */ }
fn take_func2(&'s str) { /* more lenient */ }
```

</details>

**Handling Multiple Lifetimes in Structs**

In Rust, you may encounter scenarios where a type has two distinct lifetimes. For example, one lifetime may be mutable, and the other immutable. These situations can make it necessary to use multiple lifetimes to capture the full scope of the references involved.

<details><summary>Code</summary>

```rust
struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

fn main() {
    let mut r: &str = "hello";  // &'static str -> &'a str

    *MutStr { s: &mut r }.s = "world";  // Using multiple lifetimes
    println!("{}", r);  // Prints "world", because 'b's lifetime allows for this modification
}
```

</details>

In this example, the `MutStr` struct contains two lifetimes: `'a` for the mutable reference and `'b` for the immutable reference. The lifetime `'a` allows the modification of `r` via a mutable reference, while `'b` ensures that the reference can persist even after modifications. If you attempt to simplify this to a single lifetime, the mutable reference would not work because its lifetime would be shortened, preventing further operations.

### Memory Layout and Alignment

In Rust, types serve the purpose of interpreting bits in memory. For example, the bit sequence `0b10111101` can be interpreted as the number 189 when viewed as an unsigned 8-bit integer (`u8`), or as -67 when viewed as a signed 8-bit integer (`i8`).

**Memory Alignment**

Alignment refers to how data is aligned in memory, determining where the bytes of a particular type can be stored. All values, regardless of type, must at least be byte-aligned, meaning they must be stored at addresses that are multiples of 8 bits (1 byte). On a 64-bit CPU, most values are accessed in blocks of 8 bytes (8-byte alignment), also referred to as the word size of the CPU. Ensuring that data is aligned to the CPU's _native alignment_ can improve performance because misaligned data access—when a value like an `i64` starts in the middle of an 8-byte block—requires multiple memory operations. For instance, misaligned access requires reading the first half of the value in one operation and the second half in another, leading to inefficiency. Native-aligned values are those where the alignment matches the size of the data being accessed. For example, an 8-byte value must be stored at an 8-byte aligned address to be efficiently loaded in one operation.

The compiler automatically assigns an alignment to types. For primitive types, the alignment typically matches their size. For instance, `u8` is aligned to 1 byte. More complex types that contain multiple fields (like structs) are usually aligned based on the largest field within the type. For example, a struct containing a `u8`, `u16`, and `u32` would have an alignment of 4 bytes, corresponding to the largest field (`u32`).

**Memory Layout**

The memory layout of types in Rust can be controlled using the `repr` attribute, which allows you to specify how a type should be represented in memory.

- **`repr(C)`**: This layout is compatible with C and C++ compilers. It ensures that Rust types are laid out in memory in the same way as their equivalents in C or C++.
- **`repr(transparent)`**: This layout is used for types that contain only a single field, guaranteeing that the outer type has the same layout as the inner field.
- **`repr(Rust)`**: This is Rust’s default memory layout. It optimizes memory by reordering fields for better alignment, placing larger fields first. This layout is more flexible than `repr(C)` but lacks guarantees about field order.
- **`repr(packed)`**: This layout removes padding between fields, which can save memory but may result in less efficient memory access due to potential misalignment.
- **`repr(align(n))`**: This specifies a custom alignment for types or fields, often used to ensure that different values are stored in separate cache lines, preventing issues like _false sharing_ (where multiple CPU cores access the same cache line even if they're only using different values stored within it).

**Complex Type Memory Layout**

- **Tuples**: The elements of a tuple are stored in memory in the same order as they are declared.
- **Arrays**: Elements of an array are stored contiguously, without padding.
- **Unions**: The alignment of a union is determined by the largest field within the union.
- **Enums**: The alignment of an enum is determined by its largest variant. In addition, Rust stores a hidden field in enums that acts as a _discriminant_, which indicates which variant of the enum is currently being used. The size of this discriminant depends on the number of variants in the enum.

**Dynamically Sized Types and Wide Pointers**

In Rust, most types implement the `Sized` trait, meaning their size is known at compile-time. By default, type bounds in Rust assume that the type is `Sized` (i.e., `T: Sized`). However, for types whose size is only known at runtime, you can opt out of this requirement by using the `T: ?Sized` bound.

Dynamically Sized Types (DSTs), such as _trait objects_ or _slices_, are examples of types whose size is unknown at compile time. To handle DSTs, Rust uses **wide pointers** (or _fat pointers_), which contain additional metadata. Wide pointers are `Sized` themselves because their size is always twice that of a regular pointer (`usize`). A wide pointer consists of one `usize` for the actual pointer to the data and another `usize` for the associated metadata. For example, a wide pointer to a slice contains the pointer to the slice's data and an additional `usize` for the slice's length.

When a function takes a DST as an argument, such as a slice or a trait object, the compiler automatically constructs a wide pointer. Wide pointers can be stored in types like `Box` or `Arc`, which support storing DSTs. For slices, the extra metadata in the wide pointer is the length of the slice.

In this way, Rust ensures that even types without a known size at compile time can still be handled safely and efficiently.

### Compilation and Dispatch of Trait Bounds

**Static Dispatch**

Static dispatch refers to how generic code or methods on `dyn Trait` are compiled in Rust. During compilation, the compiler generates a copy of the code for each concrete type that is used, such as `Vec<i32>`. Essentially, for every instance of a generic, the `T` is replaced with a specific type, like `i32`. Static dispatch means that the address of the method or function call is known at compile-time.

<details><summary>Code</summary>

```rust
impl String {
    pub fn contains(&self, p: impl Pattern) -> bool {
        p.is_contained_in(self)
    }
}
```

</details>

- **Why is this code copied for each type of `Pattern`?**
  - The CPU needs to know the exact address to jump to when calling the `is_contained_in` method. This means that, for every specific type implementing `Pattern`, the compiler generates a separate copy of the method with its own address.
  - Static dispatch is where the method's address is known and "dispatched" at compile time, making the calls efficient.

**Monomorphization**

Monomorphization converts generic types into specific non-generic types at compile time. The advantage is that each instance is fully optimized with the known types. However, the downside is that it increases compilation time, program size, and can reduce CPU cache efficiency due to separate machine code for each instance.

**Dynamic Dispatch**

Dynamic dispatch, on the other hand, allows you to call methods on a trait without knowing the specific type at compile-time. This is accomplished through a **vtable** (virtual method table), which holds pointers to the trait’s method implementations. When a trait method is invoked dynamically, the program looks up the correct method address in the vtable and jumps to it.

<details><summary>Code</summary>

```rust
impl String {
    pub fn contains(&self, p: &dyn Pattern) -> bool {
        p.is_contained_in(&*self)
    }
}
```

</details>

- When using `&dyn Pattern`, the caller provides two things:
  - The address of the `Pattern` implementation.
  - The address of the `is_contained_in` method.
  - These are both stored in the vtable.

With dynamic dispatch, the method to be called isn’t determined until runtime. The tradeoff is increased flexibility at the cost of some performance overhead due to the indirect lookup.

**Object Safety**

In Rust, not all traits can be converted into trait objects. **Object-safe traits** are those that can be turned into trait objects (like `dyn Trait`), while object-unsafe traits cannot. For example, traits like `Clone` (because its `clone` method returns `Self`) and `Extend` are not object-safe.

For a trait to be object-safe:

- None of its methods can be generic.
- It cannot use `Self` in method signatures, except in specific ways.
- It cannot have static methods.

Since trait objects rely on dynamic dispatch, object safety ensures that the trait can be represented using a vtable.

### Generic Traits

Rust traits can have generic parameters, and they can be written in two forms:

1. **Generic type parameters**: `trait Foo<T>` (one implementation per type).
2. **Associated types**: `trait Foo { type Bar; }` (allows multiple implementations).

It’s generally recommended to use associated types when possible.

Here’s why:

- With generic type parameters, every implementation must specify all the generic parameters, which can lead to repeated bounds and more difficult type inference.
- Associated types, on the other hand, are more concise and place the bounds within the trait itself, making the code cleaner and reducing ambiguity. However, it is not possible to implement `Deref` for multiple target types, nor can you implement `Iterator` for multiple item types.

<details><summary>Code</summary>

```rust
trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}
```

```rust

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

```

```rust
pub trait Deref {
		type Target: ?Sized;

		fn deref(&self) -> &Self::Target;
}

```

```rust

pub trait Iterator {
		type Item;
}

```

</details>

In contrast, with generic parameters, ambiguity can arise, requiring disambiguating function calls like `FromIterator::<u32>::from_iter` to resolve which instance of the trait to use.

### Orphan Rules and Coherence

**Coherence** ensures that for any given type and method, there is only one correct implementation. The **orphan rule** states that you can only implement a trait for a type if either the trait or the type is local to your crate.

There are exceptions to the orphan rule:

- **Blanket implementations**: Traits can be implemented for all types using a broad rule, like `impl<T> MyTrait for T where T`.
  - Only the crate that defines the trait can add blanket implementations. Adding one to an existing trait is a breaking change.
- **Fundamental types**: Special types like `&`, `&mut`, and `Box` are considered fundamental, meaning they are exceptions to the orphan rule. Implementing a blanket implementation for fundamental types can also be a breaking change.

- **Covered implementations**: This allows implementing external traits on external types under certain conditions. If at least one generic type in the implementation is local, the rule can be relaxed.

<details><summary>Code</summary>

```rust
// Allowed implementations
impl<T> From<T> for MyType;
impl<T> From<MyType> for Vec<T>;
impl<T> ForeignTrait<MyType, T> for Vec<T>;

// Not allowed
impl<T> ForeignTrait for T;
impl<T> From<T> for T;
```

</details>

This system ensures that changes to traits and types are non-breaking as long as they follow these rules.

### Conclusion

Rust combines flexibility with safety through its use of static and dynamic dispatch, monomorphization, object safety, and coherence rules. By understanding these concepts, you can write more efficient and maintainable Rust code, leveraging traits effectively across different types and scenarios.

<div  align="center"> 
<img src="images/06_type.png" width="100%" />
</div>
