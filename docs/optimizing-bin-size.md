```toml
[profile.release]
opt-level = "z" # Optimize for size.
lto = true # Enable Link Time Optimization (for faster builds).
codegen-units = 1 # Reduce number of codegen units to increase optimizations.
panic = "abort" # Abort on panic.
```
