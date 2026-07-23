// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers: Vec<i16> = Vec::new();

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}


// Rust Generics — Basic Tutorial
// Generics let you write one definition that works for many types, while staying type-safe at compile time.

// 1. Generic functions
// Same logic, different input types:

// fn print_twice<T: std::fmt::Debug>(x: T) {
//     println!("{:?} {:?}", x, x);
// }
// fn main() {
//     print_twice(42);           // T = i32
//     print_twice("hello");      // T = &str
// }
// <T> = “this function works for some type T”
// T: Debug = “T must implement Debug” (a trait bound)
// 2. Generic structs (like Vec)
// Vec is roughly “a growable list of T”:

// struct MyBox<T> {
//     value: T,
// }
// impl<T> MyBox<T> {
//     fn new(value: T) -> Self {
//         Self { value }
//     }
//     fn get(&self) -> &T {
//         &self.value
//     }
// }
// fn main() {
//     let b1 = MyBox::new(42);           // MyBox<i32>
//     let b2 = MyBox::new(String::from("hi"));  // MyBox<String>
//     println!("{}", b1.get());
// }
// Piece	Meaning
// struct MyBox<T>
// Struct parameterized by type T
// impl<T> MyBox<T>
// Methods for **MyBox of any T
// MyBox::new(42)
// Compiler infers T = i32
// Real Vec<T> is the same idea:

// // conceptually (simplified)
// struct Vec<T> {
//     ptr: *mut T,
//     len: usize,
//     capacity: usize,
// }
// // Vec<i16>  → vector of i16
// // Vec<String> → vector of String
// 3. Multiple generic parameters
// struct Pair<A, B> {
//     first: A,
//     second: B,
// }
// fn main() {
//     let p = Pair {
//         first: 1,
//         second: "two",
//     };
//     // Pair<i32, &str>
// }
// 4. Trait bounds (restricting T)
// “T can be anything” vs “T must support certain operations”:

// // T must be printable
// fn show<T: std::fmt::Debug>(x: T) {
//     println!("{x:?}");
// }
// // T must be orderable
// fn max<T: Ord>(a: T, b: T) -> T {
//     if a >= b { a } else { b }
// }
// Multiple bounds:

// fn dump<T: std::fmt::Debug + Clone>(x: T) {
//     let copy = x.clone();
//     println!("{copy:?}");
// }
// 5. where clause (cleaner syntax)
// fn complex<T, U>(t: T, u: U) -> String
// where
//     T: std::fmt::Debug,
//     U: std::fmt::Display,
// {
//     format!("{:?} and {}", t, u)
// }