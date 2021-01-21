# Course Syllabus

The schedule, homeworks, and syllabus are posted on the [course webpage](https://www.cis.upenn.edu/~cis198/).

# Rust Overview

"Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."

In other words: Rust is designed for speed, safety, and concurrency.

## What is "Systems Programming"?

Areas of systems programming:
- Operating Systems
- Device Drivers
- Embedded Systems
- Real Time Systems
- Networking
- Virtualization and Containerization
- Scientific Computing
- Video Games

Other systems languages include: C, C++, Ada, D.
Like C and C++, Rust gives developers fine control over the use of memory,
and maintains a close relationship between the primitive operations of the
language and those of the machines it runs on, helping developers anticipate the
costs (time and space) of operations.

## Who uses Rust?

Amazon: https://aws.amazon.com/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing

Facebook: https://developers.libra.org/

Mozilla: Firefox

Discord: https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f

Dropbox: backend infrastructure

"For five years running, Rust has taken the top spot as the most loved programming language."
- https://insights.stackoverflow.com/survey/2020#technology-most-loved-dreaded-and-wanted-languages-loved

Now a top 20 language in most language popularity rankings, and one of the fastest-growing: https://www.reddit.com/r/rust/comments/hz7dfp/rust_is_now_a_top_20_language_in_all_of_the_5/

Second most used language for Advent of Code this year, after Python: https://app.powerbi.com/view?r=eyJrIjoiZTQ3OTlmNDgtYmZlMS00ZTJmLTkwYTgtMWQyMTkxNWI5NGM1IiwidCI6IjQwOTEzYjA4LTQyZTYtNGMxOS05Y2FiLTRmOWZlM2U0YzJmZCIsImMiOjl9

## Why Rust?

    - Fast
    - Safe -> statically typed, and compile time checked for memory safety
    - Trustworthy concurrency

In particular, Rust's goals of memory safety and trustworthy concurrency are
what makes it most unique.
These concepts -- in particular the issues surrounding data ownership -- can
be both the most surprising and the most rewarding parts of learning Rust.

## Type Safety and Memory Safety

A language is said to be type-safe if all programs written in that language
have defined semantics for all possible states.

https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/

### Aside: Typing

Static Typing vs Dynamic Typing

Strongly Typed

### C and C++ are not type safe!

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

### Billion Dollar Mistake

"I call it my billion-dollar mistake. It was the invention of the null reference in 1965...
My goal was to ensure that all use of references should be absolutely safe,
with checking performed automatically by the compiler. But I couldn't resist the temptation to put in a
null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities,
and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years."

## Performance

- **Zero Cost Abstraction:** In general, C++ implementations obey the zero-overhead principle:
  What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code
  any better.

- Energy usage is also important! https://www.businessinsider.com/hhvm-saved-facebook-millions-dollars-2015-7

## Concurrency

Computers are multicore, need for parallelism
  https://www.karlrupp.net/wp-content/uploads/2015/06/35years.png

Concurrency is hard:
  https://en.wikipedia.org/wiki/Therac-25

## Examples of Rust Safety

No null pointer dereferences. (Not unique to Rust)

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

### Handling Possible Errors

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

### Memory Management

C and C++ have you handle your own memory (_manual memory management_).

Every value will live as long as it must.

Your program will never use a heap-allocated value after it has been freed.

How does Java, Python stop you from doing this?
Answer: they don't allow manual memory management.

Why not just have a garbage collector?
Time critical code, performance, runtime/portability.

# Demos

Inside `demos`:

```bash
cargo run --bin rayon
cargo run --bin reqwest
cargo run --bin structopt -- --help
```
