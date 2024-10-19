# ETAAcademy-Rust: 07. Interface

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>07. Interface</td>
    <td>
      <table>
        <tr>
          <th>rust</th>
          <th>basic</th>
          <td>Interface</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https:github.com/ETAAcademy)｜[Twitter](https:twitter.com/ETAAcademy)｜[ETA-ZK-Rust](https://github.com/ETAAcademy/ETAAcademy-Rust)

Authors: [Eta](https:twitter.com/pwhattie), looking forward to your joining

# Rust API Interfaces

**Principles of Rust API Design**

When designing Rust APIs, four core principles should be adhered to: **Unsurprising**, **Flexible**, **Obvious**, and **Constrained**.

## 1. Unsurprising

The principle of "unsurprising" means that the purpose of an interface should be predictable. This can be achieved through:

1. **Implementing Common Traits**: Commonly used traits such as `Debug`, `Send`, `Sync`, `Clone`, `Default`, `PartialEq`, `PartialOrd`, `Hash`, and `Eq` should be implemented. For instance, the `Debug` trait allows users to print types using `{:?}`, while `Serialize` and `Deserialize` from the `serde` library facilitate JSON or binary serialization.
2. **Ergonomic Trait Implementations**: Rust does not automatically implement traits for references of types that implement those traits, which can lead to surprising behaviors.

3. **Wrapper Types**: Implementing `Deref` and `AsRef` can provide inheritance-like behavior. The `Borrow` trait serves a similar purpose, allowing for the flexible handling of types.

### 1.1 Naming Conventions

The names of interfaces should adhere to conventions widely recognized in the Rust community, making their functionality clear at a glance. For example:

- An `iter` method likely takes `&self` and returns an iterator.
- A method named `into_inner` probably takes `self` and returns an underlying wrapped type.
- A type named `SomethingError` should implement the `std::error::Error` trait, appearing in various `Result` types.

### 1.2 Implementing Common Traits

#### 1.2.1 User Expectations

Users generally expect everything in the interface to "just work" as intended:

- They should be able to print any type using `{:?}`.
- Types should be safely transferable between threads.
- Every type is expected to be `Clone`.

#### 1.2.2 Recommendations for Implementing Traits

It's advisable to implement most standard traits, even if they aren't immediately necessary:

##### 1.2.2.1 **Debug Trait**: Most types should implement this, ideally through `#[derive(Debug)]`.

<details><summary><b> Code</b></summary>

```rust
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
```

</details>

</details>

- Use the various `debug_xxx` helper methods provided by `fmt::Formatter` to implement the `Debug Trait`. The available helper methods include:
  1. `debug_struct` for structs
  2. `debug_tuple` for tuples
  3. `debug_list` for lists
  4. `debug_set` for sets
  5. `debug_map` for maps

<details><summary><b> Code</b></summary>

```rust
use std::fmt;

// Define a struct Pair that holds two values of the same type.
struct Pair<T> {
    a: T,
    b: T,
}

// Implement the Debug trait for Pair<T> where T implements fmt::Debug.
impl<T: fmt::Debug> fmt::Debug for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Since Pair is a struct, we use the debug_struct method to implement it.
        f.debug_struct("Pair")    // The name is "Pair".
            .field("a", &self.a)  // Add the field "a" with its value.
            .field("b", &self.b)  // Add the field "b" with its value.
            .finish()             // Finish formatting.
    }
}

fn main() {
    // Create an instance of Pair with values 5 and 10.
    let pair = Pair { a: 5, b: 10 };

    // Print the Pair instance using the Debug trait.
    println!("Pair: {:?}", pair);
     }
```

</details>

##### 1.2.2.2 **Send and Sync Traits**: Types that are not `Send` cannot be transferred to threads, and non-`Sync` types cannot be shared via `Arc`.

<details><summary><b> Code</b></summary>

```rust
use std::rc::Rc;

#[derive(Debug)]
struct MyBox(*mut u8);

unsafe impl Send for MyBox {}

fn main() {
    let mb = MyBox(Box::into_raw(Box::new(42)));
    let x = Rc::new(42);

    std::thread::spawn(move || {
        // This will cause an error because Rc does not implement Send
        println!("{:?}", x);

        // MyBox implements Send, so this is fine
        println!("{:?}", mb);
    });
}

```

</details>

- If your type is not `Sync`, it cannot be shared through `Arc` and cannot be placed in static variables.

<details><summary><b> Code</b></summary>

```rust
use std::cell::RefCell;
use std::env::consts::ARCH;
use std::sync::Arc;

fn main() {
    let x = Arc::new(RefCell::new(42));
    std::thread::spawn(move || {
        let mut x = x.borrow_mut();
        // Since RefCell does not implement Sync, the following line causes an error
        *x += 1; // error: `RefCell<i32>` cannot be shared between threads safely
    });
 }
```

</details>

##### 1.2.2.3 **Clone and Default Traits**: Implementing `Clone` and `Default` provides a means for creating copies and default values, respectively.

<details><summary><b> Code</b></summary>

```rust
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let p1 = Person::new("Alice".to_owned(), 22);
    let p2 = p1.clone();

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
     }
```

</details>

<details><summary><b> Code</b></summary>

```rust

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point::default();
    println!("Point: ({}, {})", point.x, point.y);
     }
```

</details>

##### 1.2.2.4 **PartialEq, PartialOrd, Hash, Eq, Ord**: These traits are crucial for comparisons and hashing, particularly when types are used as keys in collections like `HashMap`.

<details><summary><b> Code</b></summary>

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };

    println!("Point1 == Point2: {}", p1 == p2);
    println!("Point1 == Point3: {}", p1 == p3);
     }
```

</details>

**`PartialOrd` and `Hash` for Specialized Use Cases**

- When a type needs to be used as a `Key` in a `Map`, it must implement `PartialOrd` to allow for comparisons of the `Key`.

<details><summary><b> Code</b></summary>

```rust
use std::collections::BTreeMap;

// Implementing these traits
// Ord requires PartialOrd
#[derive(Debug, PartialEq, Eq, Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut ages = BTreeMap::new();

    let person1 = Person {
        name: "Alice".to_owned(),
        age: 25,
    };

    let person2 = Person {
        name: "Bob".to_owned(),
        age: 23,
    };

    let person3 = Person {
        name: "Cook".to_owned(),
        age: 31,
    };

    // This will cause an error if PartialOrd is not implemented
    ages.insert(person1.clone(), "Alice's age");
    ages.insert(person2.clone(), "Bob's age");
    ages.insert(person3.clone(), "Cook's age");

    for (person, desc) in &ages {
        println!("{}: {} - {:?}", person.name, person.age, desc);
    }
 }
```

</details>

When using collection types from `std::collection` for deduplication, the type must implement `Hash` to facilitate hash calculations.

<details><summary><b> Code</b></summary>

```rust
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
    }
}

fn main() {
    let mut persons = HashSet::new();

    let person1 = Person {
        name: "Alice".to_owned(),
        age: 30,
    };

    let person2 = Person {
        name: "Bob".to_owned(),
        age: 20,
    };

    let person3 = Person {
        name: "Charlie".to_owned(),
        age: 40,
    };

    persons.insert(person1.clone());
    persons.insert(person2.clone());
    persons.insert(person3.clone());

    println!("Person Set {:?}", persons);
 }
```

</details>

**The Additional Semantic Requirements of `Eq` and `Ord`**

1. Only implement them if you are certain these semantics apply to your type.
2. They are extensions of `PartialEq` and `PartialOrd`.

- Additional requirements for `Eq`:
  1. Reflexivity: For any object `x`, `x == x` must be true.
  2. Symmetry: For any objects `x` and `y`, if `x == y` is true, then `y == x` must also be true.
  3. Transitivity: For any objects `x`, `y`, and `z`, if `x == y` is true and `y == z` is true, then `x == z` must also be true.
- Additional requirements for `Ord`:
  1. Reflexivity: For any object `x`, both `x <= x` and `x >= x` must be true.
  2. Antisymmetry: For any objects `x` and `y`, if `x <= y` and `y <= x` are both true, then `x == y` must be true.
  3. Transitivity: For any objects `x`, `y`, and `z`, if `x <= y` and `y <= z` are true, then `x <= z` must also be true.

**Recommendation to Implement `Serialize` and `Deserialize` with `serde`**

- The `serde_derive crate` provides mechanisms to override serialization for individual fields or enum variants.
  1. Since `serde` is a third-party library, you may not want to enforce its dependency.
  2. Most libraries opt to provide a `serde` feature that only adds support when users enable it.

<details><summary><b> Code</b></summary>

```rust
// library's Cargo.toml
[dependencies]
serde = { version = "1.0", optional = true }

[features]
serde = ["serde"]

// When others use your library, the feature "serde" must be enabled.
[dependencies]
mylib = { version = "0.1", features = ["serde"]  }
```

</details>

**Why Not to Implement `Copy`**

- Users typically do not expect types to be `Copy`; if they want two copies, they usually prefer to call `Clone`.
- `Copy` alters the semantics of moving a given type value, which can lead to unexpected user experiences.
- `Copy` types come with many restrictions; a type that starts as simple can quickly become unsuitable for `Copy` if it later holds more complex types like `String`.

<details><summary><b> Code</b></summary>

```rust
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // this is a Copy, not a Move.

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
 }
```

</details>

### 1.3 Ergonomic Trait Implementations

When defining a new trait, ensure implementations are provided for:

- `&T where T: Trait`
- `&mut T where T: Trait`
- `Box<T> where T: Trait`

This guarantees that references can also utilize the implemented traits.

### 1.4 Wrapper Types

Rust does not support traditional inheritance, but traits like `Deref` and `AsRef` can simulate similar functionality:

1. **Deref**: Allows calling methods on the inner type directly.

<details><summary><b> Code</b></summary>

```rust
use std::ops::Deref;

// Define a MyVec struct that contains a Vec<i32>.
struct MyVec(Vec<i32>);

// Implement the Deref trait for MyVec, targeting Vec<i32>.
impl Deref for MyVec {
    type Target = Vec<i32>;

    // The deref method returns a reference to the inner Vec<i32>.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // Create an instance of MyVec containing a vector of integers.
    let my_vec = MyVec(vec![1, 2, 3, 4, 5]);

    //  Since MyVec implements Deref, we can directly call the len() method of Vec<i32> on an instance of MyVec, and also access the first element using indexing
    println!("Length: {}", my_vec.len());
    println!("First element: {}", my_vec[0]);
}


```

</details>

2. **AsRef**: Facilitates easy conversion between types.

   - If you provide transparent types like Arc, implementing Deref allows the wrapped type to be automatically dereferenced when using operators.

   - Implementing AsRef allows users to easily convert a &WrapperType to a &InnerType.

<details><summary><b> Code</b></summary>

```rust
use std::ops::Deref;

#[derive(Debug)]
struct Wrapper(String);

impl Deref for Wrapper {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Wrapper {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Wrapper {
    fn from(s: String) -> Self {
        Wrapper(s)
    }
}

impl From<Wrapper> for String {
    fn from(wrapper: Wrapper) -> Self {
        wrapper.0
    }
}

fn main() {
    // Create a Wrapper instance from a String.
    let wrapper = Wrapper::from("Hello".to_string());

    // Use the . operator to call a method on the inner String.
    println!("Length: {}", wrapper.len());

    // Use the as_ref method to convert Wrapper to a &str type.
    let inner_ref: &str = wrapper.as_ref();
    println!("Inner: {}", inner_ref);

    // Convert Wrapper to its inner String type.
    let inner_string: String = wrapper.into();
    println!("Inner String: {}", inner_string);

    // Uncommenting the following line will convert a String directly into a Wrapper.
    // let w2: Wrapper = "World".to_string().into();
    // println!("w2 Wrapper: {:?}", w2);
}


```

</details>

3. **Borrow Trait**: This trait is specialized for situations where the type can be treated as another type, allowing for flexible handling of equivalent types.

<details><summary><b> Code</b></summary>

```rust
use std::borrow::Borrow;

fn print_length<S>(string: S) where S: Borrow<str>, {
    println!("Length: {}", string.borrow().len());
}

fn main() {
    let str1: &str = "Hello";
    let string1: String = String::from("World");

    print_length(str1);
    print_length(string1);
     }
```

</details>

## 2. Flexibility in Rust: Using Contracts, Generics, and Dynamic Dispatch

When designing software, **restrictions** (e.g., constraints such as `Trait` bounds or parameter types) and **promises** (guarantees like return types) play a crucial role. Rust provides several ways to make functions more flexible through generics, dynamic dispatch (using the `dyn` keyword), and ownership management (like the `Cow` type). This article explores best practices for building flexible Rust interfaces while balancing performance and usability.

**Designing with Contracts in Mind**

Every piece of code implicitly establishes a **contract** that defines:

1. **Restrictions:** Constraints on how the code can be used.
2. **Promises:** Guarantees made by the code, such as return types or side effects.

### 2.1 Principles for Interface Design

- Avoid imposing unnecessary **restrictions**. Only promise what you can guarantee.
- Avoid **increasing restrictions** or **removing promises** in future versions, as this may lead to breaking changes.
- Start with **looser contracts** when designing interfaces and add stricter rules or promises only when necessary. This helps maintain backward compatibility.

### 2.2 Restrictions and Promises in Practice

- **Common Restrictions in Rust:**

  1. `Trait` bounds
  2. Parameter types

- **Common Promises in Rust:**
  1. Return types from functions and methods
  2. Implementations of traits

**Example 1: A Restrictive Contract**

<details><summary><b> Code</b></summary>

```rust
fn frobnicate1(s: String) -> String {
    s

```

</details>

This function’s contract:

- It **requires** the caller to pass an owned `String`.
- It **promises** to return an owned `String`.  
  Changing this function to avoid heap allocation in the future would break its contract.

**Example 2: A Looser Contract Using `Cow`**

<details><summary><b> Code</b></summary>

```rust
use std::borrow::Cow;

fn frobnicate2(s: &str) -> Cow<'_, str> {
    Cow::Borrowed(s)
 }
```

</details>

- **Accepts** a string slice (`&str`), which doesn’t require ownership.
- **Promises** to return either a reference or an owned `String`.  
  This design is more flexible but still somewhat rigid.

**Example 3: A More Flexible Contract with Generics**

<details><summary><b> Code</b></summary>

```rust
use std::borrow::Cow;

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
```

</details>

This function:

- **Requires** a type that implements `AsRef<str>`.
- **Promises** to return the same type it received.

> There’s no universally "better" way to design a function contract. However, starting with more flexible designs helps avoid breaking changes later.

### 2.3 Using Generic Arguments to Increase Flexibility

Generics allow functions to operate on a variety of types, making interfaces more adaptable.

**Example: A Function with Generics**

<details><summary><b> Code</b></summary>

```rust
// There is a function that accepts a parameter of type AsRef<str>
fn print_as_str<T>(s: T) where T: AsRef<str> {
    println!("{}", s.as_ref());
}

//  This means it will be monomorphized for every type you use that implements AsRef<str>.
fn main() {
    let s = String::from("hello");
    let r = "world";

    print_as_str(s); // Calls print_as_str::<String>
    print_as_str(r); // Calls print_as_str::<&str>
     }
```

</details>

This generic function will be **monomorphized** at compile time, meaning a unique version of `print_as_str` will be generated for every concrete type (e.g., one for `String`, one for `&str`).

**Problem: Increased Binary Size**

If the function is called with multiple types, the binary will contain multiple versions of the function, increasing its size.

**Solution: Using Dynamic Dispatch**

To avoid this, use **dynamic dispatch** with trait objects:

<details><summary><b> Code</b></summary>

```rust
fn print_as_str(s: &dyn AsRef<str>) {
    println!("{}", s.as_ref());
}

fn main() {
    let s = String::from("hello");
    let r = "world";

    print_as_str(&s);
    print_as_str(&r);
 }
```

</details>

In this case:

- The function no longer generates multiple copies at compile time.
- **At runtime**, a virtual table (vtable) is used to determine the appropriate method implementation.

**Static vs. Dynamic Dispatch**

**Static Dispatch (Generics)**

Static dispatch generates type-specific implementations at compile time, providing **maximum performance** but at the cost of increased binary size.

**Dynamic Dispatch (Using `dyn`)**

Dynamic dispatch determines the method to invoke **at runtime**. This approach adds a small performance overhead but reduces binary size by avoiding code duplication.

<details><summary><b> Code</b></summary>

```rust
/*
    Static Dispatch:
    Let's assume we have a generic function called `process` that takes a type parameter `T` and performs some operations on it.
    This function uses static dispatch, which means that for each concrete type `T`, the corresponding implementation will be generated at compile time.
*/
fn process<T>(value: T) {
    // Code to process value
    println!("Processing T");
}

/*
    Dynamic Dispatch (using the `dyn` keyword):
    This allows for selecting an implementation at runtime.
    You can achieve this by passing trait objects as parameters,
    which is demonstrated in the example below.
*/
trait Processable {
    fn process(&self);
}

#[derive(Debug)]
struct TypeA;
impl Processable for TypeA {
    fn process(&self) {
        println!("Processing TypeA");
    }
}

struct TypeB;
impl Processable for TypeB {
    fn process(&self) {
        println!("Processing TypeB");
    }
}

/*
    This function accepts a type that is a trait object,
    requiring the object to implement the `Processable` trait.
    If a caller wants to use dynamic dispatch and choose an implementation at runtime,
    they can call the `process_trait_object` function and pass a trait object as an argument.
    This allows the caller to select the specific implementation they want to provide (protocol-oriented programming).
*/
fn process_trait_object(value: &dyn Processable) {
    value.process();
}

fn main() {
    let a = TypeA;
    let b = TypeB;

    // Calling with dynamic dispatch
    process_trait_object(&a);
    process_trait_object(&b);

    // Calling with static dispatch, where different types produce different implementations
    process(&a);
    process(&b);
    process(&a as &dyn Processable); // When using generics, the caller can always opt for dynamic dispatch by passing a trait object
    process(&b as &dyn Processable);

    println!("TypeA: {:?}", a);
     }
```

</details>

In the above example:

- **Static dispatch** occurs when calling `process`.
- **Dynamic dispatch** occurs when calling `process_trait_object`.

**When to Use Dynamic Dispatch**

Dynamic dispatch is ideal when:

- **Performance is not critical** (e.g., less frequent calls).
- You want to avoid **large binary sizes** caused by monomorphization.

However, avoid using dynamic dispatch in **hot loops** or performance-sensitive code, as the overhead might degrade performance.

**Guidelines for Using Generics and Trait Objects**

- Start with **concrete types** when designing interfaces.
- Gradually **generalize** your interface with generics if users frequently need other types.
- **Use dynamic dispatch** when the performance cost is negligible, especially if reducing binary size is important.

> Note: Rust currently only supports dynamic dispatch for simple trait bounds, like `impl AsRef<str>`. More complex trait combinations (e.g., `&dyn Hash + Eq`) are not supported by dynamic dispatch.

<details><summary><b> Code</b></summary>

```rust
fn foo(v: &Vec<usize>) {
    // Function that takes a reference to a Vec<usize>
    // Code...
}

// Change the function to accept any type that implements AsRef<[usize]>
fn foo2(v: impl AsRef<[usize]>) {
    // Function that takes a type implementing AsRef<[usize]>
    // Code...
}

fn main() {
    let iter = vec![1, 2, 3].into_iter();

    /*
        When calling `foo`, there is no issue because the compiler can infer that the result of `iter.collect()`
        should be collected as a `Vec<usize>`, since it is passed to the `foo` function that accepts `&Vec<usize>`.
    */
    // foo(&iter.collect());

    /*
        However, after changing to `foo2`, the compiler only knows that `foo2` accepts a type
        that implements the `AsRef<[usize]>` trait.
        There are multiple types that satisfy this condition, such as `Vec<usize>` and `&[usize]`.
        Therefore, the compiler cannot determine which specific type to interpret the result of `iter.collect()` as,
        leading to an inability to infer the type and causing a compilation failure for the caller's code.

        To resolve this issue, you need to specify a concrete type. For example, you can call:
        foo2(&iter.collect::<Vec<usize>>())
        or
        let list: Vec<usize> = iter.collect();
        foo2(&list);
    */
    // let list: Vec<usize> = iter.collect();
    // foo2(&list);
    foo2(&iter.collect::<Vec<usize>>());
}


```

</details>

### 2.4 Object Safety

When defining a trait in Rust, determining whether it is **object-safe** is an essential but often implicit part of the contract.

**Object safety** determines whether a **trait** can be safely used to create a **trait object**.

**A trait is object-safe if it meets the following conditions (RFC 255):**

1. **All its supertraits** must be object-safe.
2. The trait cannot require **`Self: Sized`** (i.e., it must not have the `Sized` bound).
3. **No associated constants** are allowed.
4. **No associated types with generics** are permitted.
5. **All associated functions** must meet at least one of the following criteria:
   - **Dispatchable Functions** (functions that can be dispatched via a trait object):
     - Must not have type parameters (lifetimes are allowed).
     - Only use `Self` in the receiver type (i.e., the part before the function body).
     - The receiver type must be one of the following:
       - `&Self` (i.e., `&self`)
       - `&mut Self` (i.e., `&mut self`)
       - `Box<Self>`
       - `Rc<Self>`
       - `Arc<Self>`
       - `Pin<P>`, where `P` is one of the types above.
     - Must not have a `where Self: Sized` constraint.
   - **Non-Dispatchable Functions**:
     - These functions require a `where Self: Sized` constraint.

**Why is Object Safety Important?**

- If a **trait is object-safe**, it allows you to use **`dyn trait`** to treat multiple concrete types as a single generic type.
- If a **trait is not object-safe**, the compiler will prevent the use of `dyn trait`.

**Design Tip**

When designing traits, it is recommended to strive for **object safety**. Even though it might slightly reduce usability, it provides more flexibility in how the trait can be used.

**Example of an Object-Safe Trait**

<details><summary><b> Code</b></summary>

```rust

trait Animal {
    fn name(&self) -> &str;
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) {
        println!("woof!");
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) {
        println!("Meow!");
    }
}

fn main() {
    let dog = Dog { name: "Fido".to_string() };
    let cat = Cat { name: "Whiskers".to_string() };

    let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    for animal in animals {
        println!("This is {}", animal.name());
        animal.speak();
    }
}
/**
 * - In this example, `Animal` is object-safe since it has no associated constants, generics, or `Self` return types.
 * Thus, we can store different types (`Dog` and `Cat`) in a `Vec<&dyn Animal>` and call their methods via the trait object.
 */
```

</details>

**Example of a Non-Object-Safe Trait**

<details><summary><b> Code</b></summary>

```rust
/**
 * This `Animal` trait is **not object-safe** because the `clone` method returns `Self`. As a result, it cannot be used as a trait object with `dyn Animal`.
 */
trait Animal {
    fn name(&self) -> &str;
    fn speak(&self);
    fn clone(&self) -> Self;  // Problem: Returns Self.
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) {
        println!("woof!");
    }

    fn clone(&self) -> Self {
        Dog { name: self.name.clone() }
    }
 }
```

</details>

### Solution: Adding a `Sized` Constraint

To keep the `clone` method but ensure object safety, we can **restrict it to concrete types** by adding a `where Self: Sized` constraint:

<details><summary><b> Code</b></summary>

```rust
// Now, `clone` can only be called on concrete types, not on trait objects.

trait Animal {
    fn name(&self) -> &str;
    fn speak(&self);
    fn clone(&self) -> Self where Self: Sized; // 添加 Sized 约束
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) {
        println!("wang wang!");
    }

    fn clone(&self) -> Self where Self: Sized, {
        todo!()
    }
}


#[derive(Debug)]
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn speak(&self) {
        println!("miao miao!");
    }

    fn clone(&self) -> Self where Self: Sized, {
        Cat {
            name: self.name.clone(),
        }
    }
}

fn main() {
    let dog = Dog { name: "Fido".to_string() };
    let cat = Cat { name: "Whiskers".to_string() };

    let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    for animal in animals {
        println!("This is {}", animal.name());
        animal.speak();
    }

    // `clone` can only be used with concrete types:
    let cloned_cat = cat.clone();
    println!("Cloned cat: {:?}", cloned_cat);
 }
```

</details>

**Generic Methods in Object-Safe Traits**

If you need to use **generic parameters** in a trait, consider moving the generics to the trait definition:

<details><summary><b> Code</b></summary>

```rust
trait Container<T> {
    fn contains(&self, item: &T) -> bool;
}

impl<T> Container<T> for Vec<T>
    where T: PartialEq,
{
        fn contains(&self, item: &T) -> bool {
            self.iter().any(|x| x == item)
        }
}

impl<T> Container<T> for HashSet<T>
where T: Hash + Eq,
{
    fn contains(&self, item: &T) -> bool {
        self.contains(item)
    }
}

fn main() {
    let vec_container: Box<dyn Container<i32>> = Box::new(vec![1, 2, 3]);
    let set_container: Box<dyn Container<i32>> = Box::new(
        vec![4, 5, 6].into_iter().collect::<std::collections::HashSet<_>>()
    );

    println!("Vec contains 2: {}", vec_container.contains(&2));
    println!("HashSet contains 6: {}", set_container.contains(&6));
 }
```

</details>

Alternatively, **dynamic dispatch** can sometimes be used to replace generics:

<details><summary><b> Code</b></summary>

```rust
use std::fmt::Debug;

/*
    We have a trait `Foo` that contains a generic method `bar`, which takes a generic parameter `T`:
    trait Foo {
        fn bar<T>(&self, x: T);
    }
    Is `Foo` object-safe? The answer is: it depends on the type of `T`.
    1. If `T` is a concrete type, such as `i32` or `String`, then it is `not` object-safe,
       because it requires knowing the specific type of `T` at runtime to call the `bar` method.

    2. If `T` is a trait object, such as `&dyn Debug` or `&dyn Display`, then the trait is object-safe,
       because it allows for dynamic dispatch when calling methods on `T`.
       Therefore, it can be defined as follows:
*/

trait Foo {
    fn bar(&self, x: &dyn Debug);
}

// Define struct `A` and implement the `Foo` trait for it
struct A {
    name: String,
}

impl Foo for A {
    fn bar(&self, x: &dyn Debug) {
        println!("A {} says {:?}", self.name, x);
    }
}

// Define struct `B` and implement the `Foo` trait for it
struct B {
    id: i32,
}

impl Foo for B {
    fn bar(&self, x: &dyn Debug) {
        println!("B {} says {:?}", self.id, x);
    }
}

fn main() {
    // Create trait objects using `Foo`

    let a = A { name: "Bob".to_string() };
    let b = B { id: 42 };

    // Create a Vec that stores trait objects of type `Foo`
    let foos: Vec<&dyn Foo> = vec![&a, &b];

    // Iterate over the Vec and call the `bar` method on each trait object
    for foo in foos {
        // The `&` operator allows `&str` to be treated as `&dyn Debug`
        foo.bar(&"Hello"); // The string "Hello" implements the Debug trait
    }
 }
```

</details>

### 2.5 Borrowed vs. Owned Data

In Rust, every function, trait, or type must decide **whether to own data or borrow it**.

**If the Code Needs to Own Data**

- The code must store and manage the data.
- Callers must pass owned data (not references or clones) to ensure **control over allocation and visibility of costs**.

**If Ownership Is Not Needed**

- The code should operate on **references**.
- However, small types like `i32`, `bool`, or `f64` can be copied without significant cost.

**The `Cow` (Clone-on-Write) Type**

When ownership is needed only in some cases, **`Cow`** allows data to be either **borrowed** or **owned** as required.

<details><summary><b> Code</b></summary>

```rust
use std::borrow::Cow;

/*
    The `process_data` function is designed to handle string inputs flexibly.
    It accepts a parameter of type `Cow<str>`, which allows for both owned and borrowed string data.

    There are two primary scenarios for using this function:

    1. **Modification Required:**
        - If the input string contains the term "invalid", the function needs to modify it.
        - In this case, it uses the `into_owned` method to take ownership of the string, enabling further modifications.

    2. **Read-Only Access:**
        - If the string does not contain "invalid", the function simply reads the input without modification.
*/

fn process_data(data: Cow<str>) {
    if data.contains("invalid") {
        // If the input string contains "invalid", we need to modify it.
        let owned_data: String = data.into_owned(); // Take ownership of the data.
        println!("Processed data: {}", owned_data); // Output modified data.
    } else {
        // If the input string does not contain "invalid", we only need to read it.
        println!("Data: {}", data); // Output the data as-is.
    }
}

/*
    In this example, the `Cow<str>` type allows callers to provide either:

    - A borrowed string reference (`&str`), or
    - An owned string (`String`).

    This flexibility makes the function adaptable to different usage contexts.
*/

fn main() {
    let input1 = "This is valid data.";
    process_data(Cow::Borrowed(input1));       // Pass a borrowed reference.

    let input2 = "This is invalid data.";
    process_data(Cow::Owned(input2.to_owned())); // Pass an owned string.
}


```

</details>

**Handling Complex Lifetimes and Ownership**

If lifetimes make the interface too complex, consider **owning data**—even if not strictly necessary. This approach can:

- Improve usability by reducing lifetime-related compilation issues.
- Be efficient when working with small, clonable data that isn't performance-sensitive.

By carefully balancing object safety and ownership, you can design flexible, powerful traits that offer both **usability and performance**.

### 2.6 The Impact of Fallible and Blocking Destructors on Interface Flexibility

In Rust, **destructors** are responsible for executing specific cleanup tasks when a value is destroyed. These destructors are typically implemented through the `Drop` trait, which defines a `drop` method to handle cleanup. However, destructors in Rust are expected to be infallible (i.e., they should not fail) and should avoid blocking operations. But real-world scenarios can make this challenging. This article explores the complications of fallible and blocking destructors and discusses potential solutions.

**The Challenge: Fallible and Blocking Destructors**

In some cases, destructors may need to perform tasks that could fail or block. Examples include:

- **Releasing resources:** Tasks such as closing network connections or writing logs might encounter errors.
- **Blocking operations:** Some cleanup processes require waiting for threads to terminate or waiting on asynchronous tasks. For example:
  - Flushing unsaved data to disk.
  - Closing open files or disconnecting from a network.

If such tasks fail within the destructor, the program cannot return an error to the caller directly since the `Drop` trait doesn’t allow propagating errors. In the context of asynchronous code, the situation becomes more complicated: blocking an asynchronous executor in a destructor can lead to deadlocks or stalled operations.

**A Pragmatic Approach to Cleanup with Destructors**

The recommended approach is to **perform best-effort cleanup**. If an error occurs during cleanup, it is typically ignored to avoid program disruption. Alternatively, if an executor is available, a `Future` could handle the cleanup asynchronously—though the `Future` might never run under certain conditions.

Developers often provide **explicit destructors** to avoid loose threads or unhandled cleanup. This explicit destructor can:

1. Take ownership of the resource.
2. Return any errors encountered during cleanup using `Result`.
3. Use `async fn` for asynchronous cleanup tasks.

**Example: Implementing an Explicit Destructor**

Below is an example of a custom `File` struct with an explicit `close` method for safely releasing resources and handling errors:

<details><summary><b> Code</b></summary>

```rust
/*
This design ensures that the `close` method handles all cleanup operations, including flushing data and closing the file. However, without calling `close` explicitly, the `Drop` trait would not report any cleanup failures.
 */
use std::os::fd::AsRawFd;

struct File {
    name: String,
    fd: i32,
}

impl File {
    fn open(name: &str) -> Result<File, std::io::Error> {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(name)?;
        let fd = file.as_raw_fd();

        Ok(File { name: name.to_string(), fd })
    }

    fn close(self) -> Result<(), std::io::Error> {
        let file: std::fs::File = unsafe {
            std::os::unix::io::FromRawFd::from_raw_fd(self.fd)
        };
        file.sync_all()?;
        file.set_len(0)?;
        file.sync_all()?;
        drop(file);

        Ok(())
    }
}

fn main() {
    std::fs::write("test.txt", "Hello, world!").unwrap();
    let file = File::open("test.txt").unwrap();

    println!("File name: {}, fd: {}", file.name, file.fd);

    match file.close() {
        Ok(_) => println!("File closed successfully"),
        Err(e) => println!("Error closing file: {}", e),
    }

    let metadata = std::fs::metadata("test.txt").unwrap();
    println!("File size: {} bytes", metadata.len());
 }
```

</details>

**The Problem with Combining Drop and Explicit Destructors**

When using the `Drop` trait, there are some limitations. Rust's `Drop::drop` method takes a `&mut self` reference, preventing you from moving or taking ownership of its fields. This makes it impossible to directly call an explicit destructor from `drop`.

**Example: Calling `close` from `Drop` (Error-Prone)**

<details><summary><b> Code</b></summary>

```rust
impl Drop for File {
    fn drop(&mut self) {
        // This will cause a compilation error since `close` takes ownership of `self`.
        let _ = self.close();
        println!("Dropping file {}", self.name);
    }
 }
```

</details>

Because `drop` only has access to a mutable reference, it cannot consume the fields or call the `close` method, leading to ownership issues.

**Potential Solutions to Ownership Issues**

Since there is no perfect solution, several approaches aim to balance safety and usability. Below are three common patterns.

**Solution 1: Using an Inner Type Wrapped in `Option`**

In this pattern, we create a struct with an `Option` that holds the actual fields. The explicit destructor can then take ownership of the inner type using `Option::take`. Since the inner type doesn’t implement `Drop`, we can safely move its fields during cleanup.

<details><summary><b> Code</b></summary>

```rust
struct File {
    inner: Option<InnerFile>,
}

struct InnerFile {
    name: String,
    fd: i32,
}

impl File {
    fn open(name: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::OpenOptions::new().read(true).write(true).open(name)?;
        let fd = file.as_raw_fd();

        Ok(File {
            inner: Some(InnerFile { name: name.to_string(), fd })
        })
    }

    fn close(mut self) -> Result<(), std::io::Error> {
        if let Some(inner) = self.inner.take() {
            println!("Closing file {}", inner.name);
            let file: std::fs::File = unsafe {
                std::os::unix::io::FromRawFd::from_raw_fd(inner.fd)
            };
            file.sync_all()?;
            drop(file);
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Already closed"))
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            println!("Dropping file {}", inner.name);
        }
    }
 }
```

</details>

This solution avoids double cleanup by using `Option::take`. However, it complicates access to fields since every field access requires checking the `Option`.

**Solution 2: Wrapping Fields in Individual `Option`s**

This approach involves wrapping each field inside its own `Option`. This allows moving individual fields during cleanup without ownership issues.

<details><summary><b> Code</b></summary>

```rust
struct File {
    name: Option<String>,
    fd: Option<i32>,
}

impl File {
    fn open(name: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::OpenOptions::new().read(true).write(true).open(name)?;
        let fd = file.as_raw_fd();

        Ok(File { name: Some(name.to_string()), fd: Some(fd) })
    }

    fn close(mut self) -> Result<(), std::io::Error> {
        if let (Some(name), Some(fd)) = (self.name.take(), self.fd.take()) {
            println!("Closing file {}", name);
            let file: std::fs::File = unsafe {
                std::os::unix::io::FromRawFd::from_raw_fd(fd)
            };
            file.sync_all()?;
            drop(file);
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Already closed"))
        }
    }
 }
```

</details>

While this solution provides flexibility, wrapping every field in `Option` can make code cumbersome.

**Solution 3: Using `ManuallyDrop`**

The `ManuallyDrop` type allows fields to bypass Rust’s automatic memory management. This gives control over when the destructor is called, though it introduces unsafe code.

<details><summary><b> Code</b></summary>

```rust
use std::mem::ManuallyDrop;

struct File {
    name: ManuallyDrop<String>,
    fd: ManuallyDrop<i32>,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            let _name = ManuallyDrop::take(&mut self.name);
            let _fd = ManuallyDrop::take(&mut self.fd);
        }
    }
 }
```

</details>

While powerful, this solution requires careful use of `unsafe` code, increasing the risk of undefined behavior.

## 3. Obvious

When designing interfaces, it’s essential to remember that **users are often unaware of the internal implementation details**. As a result, they may not fully understand all the rules and constraints of the interface.

To create a reliable interface, it should be **easy to understand and difficult to misuse**. Rust’s documentation and type system play a key role in achieving this.

### 3.1 Documentation

The first step toward a transparent interface is **writing good documentation**. Let’s explore key practices for clear documentation.

#### 3.1.1 Document Unexpected Behaviors

If the interface depends on operations beyond the type signature’s requirements, they should be documented to prevent surprises.

- **Panics**: If your code can panic, the conditions leading to a panic must be clearly stated in the documentation.
- **Errors**: If the function can return errors, specify the conditions under which it might do so.
- **Unsafe functions**: For unsafe functions, explicitly list the prerequisites the user must meet to call the function safely.

**Example: Documenting a Division Function**

<details><summary><b> Code</b></summary>

```rust
// Panics if the divisor is zero.

// let result = divide(10, 2);
// assert_eq!(result, 5);
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    todo!()
 }
```

</details>

#### 3.1.2 Provide End-to-End Use Cases

At the **crate or module level**, provide use cases showing how individual components fit together. This helps users gain a complete picture of the interface structure and ensures they understand how to use it effectively.

- **Benefits**:
  - Developers can quickly grasp the purpose of each method and type.
  - Users can copy and modify the examples as starting points for their use cases.

#### 3.1.3 Organize Documentation Effectively

Use **modules to group related items** semantically and **internal links** to connect related sections. Use the `#[doc(hidden)]` attribute to hide deprecated or internal components that are not meant for public use, helping keep the documentation clean.

**Example: Organizing Internal Components**

<details><summary><b> Code</b></summary>

```rust
pub mod internal {
    /// Helper function for internal calculations (hidden from documentation).
    #[doc(hidden)]
    pub fn internal_helper() {
        // Implementation...
    }

    /// Struct intended for internal use only.
    #[doc(hidden)]
    pub struct InternalStruct {
        // Fields and methods...
    }
}

// Public function calling an internal helper function.
pub fn public_function() {
    internal::internal_helper();
 }
```

</details>

#### 3.1.4 Enrich Documentation with Additional Resources

In some cases, you may need to explain complex concepts. **Link to external resources** such as RFCs, blog posts, or research papers to provide further reading.

- Use `#[doc(cfg(..))]` to indicate that certain features are only available under specific configurations.
- Use `#[doc(alias = "...")]` to allow users to search for types or methods by alternative names.
- Create **top-level documentation** that introduces the most important modules, traits, and methods.

**Example: Rich Documentation for a Library**

<details><summary><b> Code</b></summary>

```rust
///! This library provides essential image processing functions.
///!
///! Features:
///! - Read and save images in various formats [`Image::load`] [`Image::save`]
///! - Resize, rotate, and crop images [`Image::resize`] [`Image::rotate`] [`Image::crop`]
///! - Apply filters and effects [`Filter`] [`Effect`]
///!
///! Additional Resources:
///! - [Digital Image Processing](https://book.xxx.com/subject/xxxx) – A classic textbook
///! - [Learn OpenCV](https://learnopencv.com) – Tutorials on image processing with OpenCV
///! - [Awesome Computer Vision](https://github.com/jbhuang0604/awesome-computer-vision)
```

</details>

### 3.3 Type System

Rust’s type system ensures that interfaces are:

1. **Self-explanatory**
2. **Difficult to misuse**
3. **Clear in intent**

#### 3.3.1 Semantic Types

Some values carry meaning beyond their basic type. For instance, **`0` and `1`** may represent **male and female**. Using **boolean parameters** directly can lead to confusion, so converting them to **enums** enhances clarity and prevents misuse.

**Example: Replacing Boolean Parameters with Enums**

<details><summary><b> Code</b></summary>

```rust
// Using booleans can lead to confusion.
fn process_data(dry_run: bool, overwrite: bool, validate: bool) {
    // Code...
}

// Define meaningful enums instead.
enum DryRun { Yes, No }
enum Overwrite { Yes, No }
enum Validate { Yes, No }

// Use the enums in function signatures for clarity.
fn process_data2(dry_run: DryRun, overwrite: Overwrite, validate: Validate) {
    // Code...
}

fn main() {
    process_data2(DryRun::No, Overwrite::Yes, Validate::No);
 }
```

</details>

This approach makes the code **more readable** and prevents accidental misuse of parameters.

#### 3.3.2 Zero-Sized Types for State Management

Zero-sized types (ZSTs) can represent specific **states** of a type. For example, in a rocket system, the rocket should not be able to launch twice. By using **generic types with phantom data**, we can restrict the operations based on the state.

**Example: State-Specific Rocket Operations**

<details><summary><b> Code</b></summary>

```rust
struct Grounded;
struct Launched;

enum Color {
    White,
    Black,
}

struct Kilograms(u32);

struct Rocket<Stage = Grounded> {
    stage: std::marker::PhantomData<Stage>,
}

impl Default for Rocket<Grounded> {
    fn default() -> Self {
        Self { stage: Default::default() }
    }
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        Rocket { stage: Default::default() }
    }
}

impl Rocket<Launched> {
    pub fn accelerate(&mut self) {}
    pub fn decelerate(&mut self) {}
}

impl<Stage> Rocket<Stage> {

    pub fn color(&self) -> Color {
        Color::White
    }

    pub fn weight(&self) -> Kilograms {
        Kilograms(0)
    }

 }
```

</details>

This design enforces the correct sequence of operations, ensuring that only launched rockets can be accelerated or decelerated.

#### 3.3.3 Using `#[must_use]` for Critical Functions

The `#[must_use]` attribute instructs the compiler to warn users if they ignore the return value of a function or type. This helps prevent errors, particularly when dealing with functions that return **important results or errors**.

**Example: Using `#[must_use]` to Avoid Ignored Results**

<details><summary><b> Code</b></summary>

```rust
use std::error::Error;

#[must_use]
fn process_data(data: Data) -> Result<(), Error> {
    // Code...
    Ok(())
 }
```

</details>

If the user calls `process_data` without handling its return value, the compiler will generate a warning, encouraging proper error handling.

## 4. Constrained Interfaces

**Think Twice Before Modifying Interfaces**

When modifying interfaces visible to users, caution is necessary. Any change should:

- **Avoid breaking users' existing code**.
- **Remain stable over time**, minimizing frequent updates.

Frequent **backward-incompatible changes** (major version increments) will lead to user dissatisfaction.

## 4.1 Backward-Incompatible Changes

Some breaking changes are obvious (e.g., renaming public components), while others can be subtle, often tied to how Rust handles visibility, traits, and type structures. This section focuses on subtle changes and strategies for managing them.

### 4.1.1 Modifying Types

Removing or renaming public types will almost certainly break user code. However, you can mitigate this using **visibility modifiers**.

<details><summary><b> Code</b></summary>

```rust
/*
- Example: Rust’s visibility modifiers:
  - `pub(crate)`: Visible only within the current crate.
  - `pub(in path)`: Visible within a specific path.
  - `pub(super)`: Visible to the parent module.
 */
pub mod outer_mod {
    pub mod inner_mod {
        pub(in crate::outer_mod) fn outer_mod_visible_fn() {}

        pub(crate) fn crate_visible_fn() {}

        pub(super) fn super_mod_visible_fn() {
            inner_mod_visible_fn();
        }

        pub(self) fn inner_mod_visible_fn() {}
    }

    pub fn foo() {
        inner_mod::outer_mod_visible_fn();
        inner_mod::crate_visible_fn();
        inner_mod::super_mod_visible_fn();
        // Error! inner_mod_visible_fn is private.
        // inner_mod::inner_mod_visible_fn();
    }
}

fn bar() {
    outer_mod::inner_mod::crate_visible_fn();

    // Error! super_mod_visible_fn is private
    outer_mod::inner_mod::super_mod_visible_fn();

    // Error! outer_mod_visible_fn is private
    outer_mod::inner_mod::outer_mod_visible_fn();

    outer_mod::foo();
}

fn main() {
    outer_mod::foo();
 }
```

</details>

Reducing the exposure of public types makes future modifications easier by avoiding breakages.

#### Example 2: Hidden Type Dependencies

Sometimes user code relies not just on names but on type structures:

<details><summary><b> Code</b></summary>

```rust
// lib.rs
pub struct Unit {
    field: bool,
}

// main.rs
fn is_true(u: constrained_04::Unit) -> bool {
    matches!(u, constrained_04::Unit { field: true })
}

fn main() {
    let u = constrained_04::Unit;  // Error: field was added to Unit.
 }
```

</details>

Adding a new field will break the user’s code, even if the field is private. **The `#[non_exhaustive]` attribute** can help here:

<details><summary><b> Code</b></summary>

```rust
// lib.rs
#[non_exhaustive]
pub struct Config {
    pub window_width: u16,
    pub window_height: u16,
}

fn some_function() {
    let config = Config {
        window_width: 640,
        window_height: 480,
    };

    if let Config {
        window_width,
        window_height
    } = config {
        // ....
    }

}

// main.rs
use constrained_04::Config;

fn main() {
    let config = Config {
        window_width: 640,
        window_height: 480,
        .. // Required to match non-exhaustive struct.
    };

    if let Config {
        window_width,
        window_height,
        ..
    } = config {
        // ....
    }
 }
```

</details>

With `#[non_exhaustive]`, structs or enums are easier to extend in the future without breaking code.

### 4.2.2 Trait Implementations

Rust’s **coherence rules** prevent a type from implementing the same trait multiple times. However, several situations can lead to breaking changes:

1. **Adding a blanket implementation**:  
   Example: `impl<T> Foo for T`.
2. **Implementing external traits for your types** or vice-versa.
3. **Removing trait implementations**.

**Case 1: Conflicting Implementations**

<details><summary><b> Code</b></summary>

```rust
// lib.rs
pub struct Unit;
pub trait Foo1 {
    fn foo(&self);
}
impl Foo1 for Unit {
    fn foo(&self) {
        println!("foo1 is called");
    }
}

// main.rs
use constrained_04::{Foo1, Unit};

trait Foo2 {
    fn foo(&self);
}
impl Foo2 for Unit {
    fn foo(&self) {
        println!("foo2 is called");
    }
}

fn main() {
    // Error: Conflicting `foo` implementations from Foo1 and Foo2.
    Unit.foo();
 }
```

</details>

**Case 2:**

<details><summary><b> Code</b></summary>

```rust
// lib.rs

pub struct Unit;

pub trait Foo1 {
    fn foo(&self);
}

// case2: add a new public trait
pub trait Bar1 {
    fn foo(&self);
}

impl Bar1 for Unit {
    fn foo(&self) {
        println!("bar1");
    }
}



// main.rs

// use constrained_04::Unit;

use constrained_04::*;

// case1 & case2

trait Foo2 {
    fn foo(&self);
}

impl Foo2 for Unit {
    fn foo(&self) {
        println!("foo2 is called");
    }
}

fn main() {
    // If constrained_04::* is used, since there is foo method in Bar1 implemented in lib.rs, it was renamed so the chromosome reported an error.
    Unit.foo();

 }
```

</details>

#### Sealed Traits

To avoid breaking changes when adding methods to a trait, you can create a **sealed trait**—a trait that can’t be implemented by other crates.

<details><summary><b> Code</b></summary>

```rust
// lib.rs

use std::fmt::{Debug, Display};

mod sealed {
    use std::fmt::{Debug, Display};

    // This is the sealed trait.
    pub trait Sealed {}

    // Implement Sealed for any type T that implements both Debug and Display.
    impl<T> Sealed for T where T: Debug + Display {

    }
}

// Any type implementing CanUseCannotImplement must also implement sealed::Sealed. Below, we implement sealed::Sealed.
pub trait CanUseCannotImplement: sealed::Sealed {
    // ...
}

// Here we implement CanUseCannotImplement for any type T that is Debug and Display.
impl<T> CanUseCannotImplement for T where T: Debug + Display {

}


// main.rs

pub struct Bar {}

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
// Bar cannot implement it again.
impl CanUseCannotImplement for Bar {

}

pub struct Foo {}

// Error! Foo does not implement Debug and Display, so it cannot implement CanUseCannotImplement.
impl CanUseCannotImplement for Foo {

}

fn main() {

     }
```

</details>

Use sealed traits to control external implementations and prevent breaking changes, but note that they limit flexibility for downstream crates.

### 4.2.3 Hidden Contracts

#### Re-exports

If your interface exposes external types, any change in those external dependencies will become a breaking change for your users. Use the **newtype pattern** to wrap external types.

<details><summary><b> Code</b></summary>

```rust
// lib.rs
pub fn iter<T>() -> itercrate::Empty<T> { /*...*/ }

// main.rs
struct EmptyIterator {
    it: itercrate::Empty<()>,
}

let it = EmptyIterator { it: bestiter::iter() };
```

</details>

If the dependency `itercrate` updates from version 1.0 to 2.0, your users will face compilation issues because the compiler sees the two versions as different types.

#### Auto Traits

Some traits, such as `Send` and `Sync`, are **automatically implemented** by the compiler based on a type’s structure. Any change to the internals of a type can inadvertently remove these traits, causing subtle breakages.

<details><summary><b> Code</b></summary>

```rust
fn is_normal<T>() where T: Sized + Send + Sync + Unpin {}

#[test]
fn normal_types() {
    is_normal::<MyType>();  // Ensures MyType implements these traits.
 }
```

</details>

Testing for trait implementations can help detect these issues early.

## Conclusion

Designing robust Rust APIs requires balancing performance, usability, and maintainability. Start with flexible, generic interfaces to encourage broad usage, tightening constraints only when needed. Manage binary size by using generics carefully and leveraging dynamic dispatch to minimize duplication. Resource management, particularly with destructors, demands careful handling to balance safety and flexibility. Clear documentation, combined with an expressive type system, ensures APIs are predictable, self-explanatory, and hard to misuse. By adhering to these principles, developers can create user-friendly, efficient, and maintainable interfaces that enhance the developer experience.

<div align="center"> 
<img src="images/07_interface.gif" width="100%" />
</div>
