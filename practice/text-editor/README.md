## TUI Editor

**Buffer**

- `Editro` - mediates between UI components, transferring focus and info among them;
- `View` - used information from `Editor` (like text-related event) to enhance rendering efficiency and pass them to `Buffer`;
- `Buffer` - hold our text and manipulate it.

_Buffer_ - a common structure that holds everyting a text editor need to modify and display text.
The implementation details of a `Buffe` vary be text editory. Nano utiilzes a straightforward `Buffer` structure with no other internal representation of the document. On the other hand, vim features a sophisticated internal structure to efficiently handle operations, even on large files, and includes a separate `Scren Buffer` that represents the visible part of the document.

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

## References

- [Hecto: Building a Text Editor / flenker.blog](https://flenker.blog/hecto/)
