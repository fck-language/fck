# fck Lang files

These files are the language files used to allow fck to have multilingual support. This README outlines the layout of files and what is in each one.

## Contents
- [File names](#file-names)
- [What goes in the file](#file-contents)
- [How to add things in](#file-layout)

## File names

Each file name is the [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) language code for the language. For example `en.rs` is the language file for English

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
