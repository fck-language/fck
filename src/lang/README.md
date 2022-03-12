# fck Lang files

These files are the language files used to allow fck to have multilingual support. This README outlines the layout of files and what is in each one.

## Contents
- [File names](#file-names)
- [What goes in the file](#file-contents)
- [How to add things in](#file-layout)
- [Non-UTF-8](#non-utf-8-characters)

## File names

Each file name is the [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) language code for the language. For example `cn.rs` is the language file for Chinese

## File contents

Each language file must contain the following things:

- Keyword lists
    - Main keywords
      
      General use keywords used throughout the code
    - Variable keywords
      
      Names of the built-in variable types

## File layout

```rust
use crate::keywords::Keywords;

pub const KEYWORDS: Keywords = Keywords{
    keywords: [...],
    var_keywords: [...]
};
```

## Non-UTF-8 characters

All the files have to be in UTF-8 because Rust will have a meltdown otherwise. If you have to use non-UTF-8 characters (for example Korean characters), just type them as normal and it somehow works in the back

## Docstrings

At the top of each language file should be a docstring for that file. This should have the form
```rust
//! Language file for {language}
```
If the file is in development, you should add
```rust
//! **Development version**
```
to the bottom of the docstring (eg [`fr.rs`](src/fr.rs)). Before a release, all language files must be up-to-date and not be in active development. At release, only language files that are up-to-date and not being actively worked on will be included.
