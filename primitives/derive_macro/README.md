## About the Derive Macro

Update cargo file:

```toml
[lib]
proc-macro=true

[dependencies]
syn = { version = "2.0.100", features = ["full", "parsing"] }
```

`proc-macro` - allow creating syntax extensions as execution of a function
`syn` - a crate for working with abstract syntax trees

# Bibliography

- [ / cetra3.github.io](https://cetra3.github.io/blog/creating-your-own-derive-macro/)
- [ / doc.rust-lang.org](https://doc.rust-lang.org/reference/procedural-macros.html)
