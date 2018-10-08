* August 29
  First day of class!
** Class minutia
** Why Rust?
    - Fast
    - Safety -> statically typed, and compile time checks, safe subset
    - Concurrency
*** Rust is a systems programming language
  - C and C++, Ada and D

  - Operating Systems
  - Device Drivers
  - Embedded Systems
    https://en.wikipedia.org/wiki/Therac-25
  - Real Time Systems
  - Networking
  - Virtualization and Containerization
  - Scientific Computing
  - Video Games

  - Like C and C++, Rust gives developers fine control over the use of memory,
  and maintains a close relationship between the primitive operations of the
  language and those of the machines it runs on, helping developers anticipate their
  code’s costs.

  - Rust adds its own goals of memory safety and trustworthy concurrency.
*** Type Safety
    A language is said to be type safe, if all programs written in that language
    have defined semantics for all possible states.

    - Static Typing vs Dynamic Typing
    - Strongly Typed

    - What's the opposite of type safety? Undefined Behavior

    - Talk About undefined behavior
    file:./example1.c

    - C and C++ are not type safe!
    - Systems languages are what we built everything on top of!
      https://stackoverflow.com/questions/1220914/in-which-language-are-the-java-compiler-and-jvm-written
    - CPython

    - Why then do we not check things at runtime?

*** Performance
  - In general, C++ implementations obey the zero-overhead principle: What you don’t
  use, you don’t pay for. And further: What you do use, you couldn’t hand code any
  better.
  - Computers are multicore, need for parallelism
    https://www.karlrupp.net/wp-content/uploads/2015/06/35years.png
  - Power (W)

*** High Level Iterators
https://rust.godbolt.org/
Use -O
```rust
pub fn process_array(a: &[i32]) -> i32{
    a.iter().map(|i| 2*i).filter(|i| i % 2 == 0)
    .fold(0, |i, accum| i + accum)
}
```rust
*** Zero Cost Abstraction
     Monomorphisation
*** Safety
**** No null pointer dereferences.
     Billion Dollar Mistake
     https://en.wikipedia.org/wiki/Tony_Hoare

      Your program will not crash because you tried to dereference a null pointer.

      The problems with null
      1) Used to represent a missing value: String getValue(HashTable<String, String> t);
      2) Use to represent an error: void* malloc(size_t size)

      3) It is very easy to ignore.
      4) Program may work for a long time, until it doesn't
      5) It is useful though!

      Because no null pointers!
      Not unique to rust.

```
      Optional Values:
      enum Option<P> {
        Some(P),
        None
      }
```
      Still compiles down to pointer and null!

      ssize_t bytes_read = read(fd, buffer, sizeof(buffer));
      process_bytes(buffer, bytes_read);


      Handling Possible Errors
```
      enum Result<T, E> {
        Ok(T),
        Err(E),
      }


      fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;


      type Result<T> = std::result::Result<T, Error>

      fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
```

**** No dangling pointers. Every value will live as long as it must.
      Your program will never use a heap-allocated value after it has been freed.

      How does Java, Python stop you from doing this?

      The enemy is manual memory management.

      Why not just have a garbage collector? Embedded devices complicated,
      real time systems.

****** Rule 1: Every value has a single owner at any given time.
       You can move a value from one owner to another, but when a value’s
       owner goes away, the value is freed along with it.

       Variables own their values
       Single owner, when it goes out of scope the value is freed
```
       {
         let s = "omar".to_string();
         let s2 = s;
         let s3 = s;
       }
```
       In C++ this would copy the value.

       Do we mean move values or copy value?

       C++ Move semantics...

       // compile-time error: use of moved value: `s`
****** Rule 2: You can borrow a reference to a value, so long as the reference doesn’t outlive the value
       (or equivalently, its owner). Borrowed references are temporary pointers;
       they allow you to operate on values you don’t own.
```
       - Passing and returning arguments to a function.
       add1(v: vec<int>) -> Vec<int>;

       void add1(vector<int>& v);


       let str = "omar".to_string();

       fn f(s: &String){ ... }
       f(str)
       // Keep using str

       Primitives types don't have this issue.
       let x = 1;
       fn f(i: int);
       let y = x;

       trait Copy{

       }

       impl Copy for Person {

       }

       #[derive(Copy)]

       Clone for copying types.


       int* f(int n){
         int array[n];
         // Init to something
         return array;
       }
```
****** Rule 3: You can only modify a value when you have exclusive access to it.


****  No buffer overruns.
      Your program will never access elements beyond the end or before the start of an array.

      Heartbleed: https://xkcd.com/1354/

      Problem: C and C++ allow for direct memory dereference, no checking.

      Rust checks.
```
      fn fill(s: &mut[i32], n: i32) {
        for i in 0..s.len() {
        s[i] = n;
      }

      fn fill(a: &mut[i32], n: i32) {
        for i in a {
          *i = n;
      }
```
**** Concurrent
```
     let handle = std::thread::spawn(|| {
       println!("Hello world!");
       3
     })

     let ret = handle.join();

     {
       let mut string = "hello".to_string();
       spawn(|| {
         println!("{}", string);
       });
     }
```
     error: closure may outlive the current function, but it
     borrows `x`, which is owned by the current function
```
     {
       let mut string = "hello".to_string();
       spawn(move || {
         println!("{}", string);
       });
     }
```

     Rust complains!

     Rust doesn't know about threads, this is a direct consequence of our Rules.



     Sharing data across multiple threads,
```
     use std::thread::*;
     use std::sync::Arc;

     fn main() {
       let str2 = "omar".to_string();
       spawn(move || {
         println!("{}", str2);
       });
       spawn(move || {
         println!("{}", str2);
       });
     }


     use std::thread::*;
     use std::sync::Arc;

     fn main() {
       let str2 = Arc::new("omar".to_string());
       let child_ref = str2.clone();
       let child_ref2 = str2.clone();
       spawn(move || {
         println!("{}", child_ref);
       });
       spawn(move || {
         println!("{}", child_ref2);
       });
     }
```
