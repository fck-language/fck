# Type things

This is all the code for the type system in fck. It's a bit messy because I wasn't really sure how to implement this nicely and this was the best I could come up with.

The remainder of this document is explaining how the system works and is mostly just so I can understand what I'm coding.

## How work?

Each type has a unique ID. This is used to identify types, since using names is annoying and language-specific which don't want. These are all inside a vector, where the index in the vector is the type ID. The actual type looks like this:
```rust
pub struct fckType {
	names: HashMap<String, String>,
	functions: HashMap<String, FuncCallSig>
}
```

We can get away with using a single `String` as the key for the functions here because before we even get here, all the strings are converted into their originals. 
