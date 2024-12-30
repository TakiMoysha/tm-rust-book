## TUI Editor

**Buffer**

- `View` - used information from `Editor` (like text-related event) to enhance rendering efficiency and pass them to `Buffer`;
- `Editor` - mediates between UI components, transferring focus and info among them;
- `Buffer` - hold our text and manipulate it. Holds everything a text editor need to modify and display text.

The implementation details of a `Buffer` vary be text editory. Nano utiilzes a straightforward `Buffer` structure with no other internal representation of the document. On the other hand, vim features a sophisticated internal structure to efficiently handle operations, even on large files, and includes a separate `Scren Buffer` that represents the visible part of the document.

**Panic Behavior**

Rust gives you optiont to handle panic your way:

- _catching unwinds_ - recover from a panic by unwinding the stack;
- _creating custom panic handler_ - replace the default panic handler with your own;
  ```rust
  #[panic_handler]
  fn panic(info: &PanicInfo) -> ! {
    // handle panic
  }
  ```
- _panic hooks_ - before the panic unfolds, you can have functionality that kick in to tidy up, setting up a more controlled crash or cleanup.
- _drop trait_ - implement the `Drop` trait for structure to be called when it goes out of scope. But, `Drop` is also called during panicking. But, if `Drop` call panics, we _double panic_, which forces Rust to stop everyting and abort. So the code for `Drop` should avoid using panic.

**Struct Lifecycle**

Stages in the lifetime cycle: creation, usage, destruction.

_Creation_ - the process of creating a new instance of a struct. We can using `Default` trait for zero-argument creation.
_Usage_ - rust language is built around the concept of lifetime, this neccessary for _Memory Safety_. This meaning that the references we work with are always valid.
_Destruction_ - the process of removing things from memory. However, we somethimes have the need to do some cleanup if a `struct` of ours goes out of memory - for instance, disabling raw mode again.

When struct implements the `Copy` trait, the closure will have a copy available to work with.

```rust
fn main() {
  let msg = String::from("Hello");
  let greet = move |name| println!("{} {}", msg, name);
  greet("Zoki!");
}
```

For panic hook we using `Box`, why? Because we want our closure to survive the entire program execution process. We achieve this by putting it inside a `Box`. This boxes the closure in memory, and we pass a pointer to this boxed closure to `set_hook`.

```rust
let current_hook = std::panic::take_hook();
std::panic::set_hook(Box::new(move |info| {
  current_hook(info);
}))
```

**Flags Code Conventions**


## References

- [Hecto: Building a Text Editor / flenker.blog](https://flenker.blog/hecto/)
- [Flag Conventions / docs.launchdarkly.com](https://docs.launchdarkly.com/guides/flags/flag-conventions)
