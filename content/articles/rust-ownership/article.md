## The Borrow Checker

Rust's defining feature is the borrow checker, which guarantees memory safety at compile time.

Here is a simple example of ownership:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // This would not compile!
    println!("{}, world!", s2); // This is fine.
}
```

### Why it matters

By enforcing these rules at compile time, Rust eliminates classes of bugs such as:
- Null pointer dereferences
- Use after free
- Double free
- Data races in concurrent programming

This makes systems built in Rust incredibly resilient.
