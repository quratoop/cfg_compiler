# cfg-compiler

this crate is for use in the build script section.

# function app

The app function is used to configure the cfg setup for relaxed work with the correct compiler. It configures the following five cfg attributes (dev, beta, stable, nightly, bootstrap), which can be used to adapt the code to the corresponding compiler (e.g., you provide a function under cfg(nightly) that uses optimizations that it otherwise wouldn't be able to use). The bootstrap cfg is used to obfuscate any nightly functions to maintain stability guarantees.

```rust
// main.rs

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* ... run other stuff */
    cfg_compiler::app()?;
    Ok(())
}
```

# documentation

need someone for optimize the documentation or the README file.
