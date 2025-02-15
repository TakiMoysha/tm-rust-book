# Iterators

`Iterator` - the stateful component of iteration
`IntoIterator` - represents the capability for a type to be iterated over
`FusedIterator` - indicates that the iterator always returns `None` when exhausted

**Base Iterator**
Represents the state of iteration. Whiel not strictly necessery: the associated `IntoIterator::Item` type exists for convenience. Using the trait can directlry specify the `Item` using `impl IntoIterator<Item = Foo>`.

**Bounded Iterator**
The base iterator, potentially infinite, with provide a undifined behavior. Bounded Iterator have a `size_hint` method that returns the number of remaining elements. Implemented throught the `std::iter::ExactSizeIterator` (stable) and `std::iter::TrustedLen` (unstable).

**Fused Iterator**
Classic - return None at the "end" of the iterator. The specification indicates that `Next` can be caused after that. `FusedIterator` always return `None` when it was return first `None` value.

**Thread-Safe Iterator**
Thread-Safe iterator are obtainned by composing a `Sync` and `Send` auto-traits.

**Dyn-Compatible Iterator**
Dync-compatibility is an inherent part of the trait and is governed by `Sized` bounds. `Iterator` and `IntoIterator` traits are inherently dyn-compatible, thet means they can be used to create trait objects using the `dyn` keyword.

**Double-Ended Iterator**
Allows you to be at the end and at the start.

**Seeking Iterator**
Allows you to control the `Read` trait cursor. It may be useful when working with In-Memory collections like `Vec` and remote objects like pagination in the API.

**Compile Time Iterator**
We can use `const {}` to execute code during compilation. Only `const fn` can be used from `const {}` block.


## References

- [A Survey of Every Iterator Variant / blog.yoshuawuyts.com](https://blog.yoshuawuyts.com/a-survey-of-every-iterator-variant/)
- [Keyword dyn / doc.rust-lang.org](https://doc.rust-lang.org/std/keyword.dyn.html)
