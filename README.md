# Rust Overview

"Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."

Other systems languages include: C, C++, Ada, D.

Like C and C++, Rust gives developers fine control over the use of memory,
and maintains a close relationship between the primitive operations of the
language and those of the machines it runs on, helping developers anticipate the
costs (time and space) of operations.

Areas of systems programming:
- Operating Systems
- Device Drivers
- Embedded Systems
- Real Time Systems
- Networking
- Virtualization and Containerization
- Scientific Computing
- Video Games

## Who uses Rust?
Mozilla: Firefox

Amazon: https://aws.amazon.com/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing

Facebook: https://developers.libra.org/

Dropbox: backend infrastructure


"For the fourth year in a row, Rust is the most loved programming language"
- https://insights.stackoverflow.com/survey/2019

# Why Rust?
    - Fast
    - Safe -> statically typed, and compile time checked for memory safety
    - Concurrency

    Rust adds its own goals of memory safety and trustworthy concurrency.

# Type Safety and Memory Safety
A language is said to be type safe, if all programs written in that language
have defined semantics for all possible states.

https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/

## Aside: Typing
Static Typing vs Dynamic Typing

Strongly Typed

## C and C++ are not type safe!
Undefined Behavior is the root of all evil.

### Undefined Behavior
No buffer overruns.
Your program will never access elements beyond the end or before the start of an array.

Heartbleed: https://xkcd.com/1354/

Problem: C and C++ allow for direct memory dereference, no checking.

Systems languages are what we built everything on top of!

Even other languages are built on top of C and C++:
- Java Virtual Machine
- CPython

**Why then, do we not check things at runtime?**

## Billion Dollar Mistake

"I call it my billion-dollar mistake. It was the invention of the null reference in 1965...
My goal was to ensure that all use of references should be absolutely safe,
with checking performed automatically by the compiler. But I couldn't resist the temptation to put in a
null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities,
and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years."

# Performance
- Zero Cost Abstraction: In general, C++ implementations obey the zero-overhead principle:
  What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code
  any better.

- Energy usage is also important! https://www.businessinsider.com/hhvm-saved-facebook-millions-dollars-2015-7

# Concurrency
Computers are multicore, need for parallelism
  https://www.karlrupp.net/wp-content/uploads/2015/06/35years.png

Concurrency is hard:
  https://en.wikipedia.org/wiki/Therac-25

# Examples of Rust Safety

No null pointer dereferences. (Not unique to rust)

Your program will not crash because you tried to dereference a null pointer.

The problems with null
1) Used to represent a missing value: String getValue(HashTable<String, String> t);
2) Use to represent an error: void* malloc(size_t size)
3) It is very easy to ignore.

```rust
// Optional Values:
enum Option<P> {
  Some(P),
  None
}
```

Still compiles down to pointer and null!

## Handling Possible Errors

```rust
ssize_t bytes_read = read(fd, buffer, sizeof(buffer));
process_bytes(buffer, bytes_read);
```
```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}

fn read(&mut fd: FileDescriptor, buf: &mut [u8]) -> Result<usize, std::io::Error>;
```

## Memory Management

C and C++ have you handle your own memory (_manual memory management_).

Every value will live as long as it must.

Your program will never use a heap-allocated value after it has been freed.

How does Java, Python stop you from doing this?

The enemy is manual memory management.

Why not just have a garbage collector?
Time critical code, performance, runtime/portability.

# Move Semantics and the borrow checker

Through the semester, we will see how Rust handles memory management,
memory safety, and data race freedom through the use of the
borrow checker.
