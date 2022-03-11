# fck Lang files

These files are the language files used to allow fck to have multilingual support. This README outlines the layout of files and what is in each one.

## Contents
- [File names](#file-names)
- [What goes in the file](#file-contents)
- [How to add things in](#file-layout)
- [UTF-16](#UTF-16-strings)

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

## UTF-16 strings

Rust isn't a fan of any characters that can't be placed into a `u8`. Because of this, if you write a language file for anything that uses non-UTF-8 characters (such as Korean or Japanese), rust won't compile. To get around this, any language file using UTF-16 must be written in UTF-8, where each 16-bit character is written as two 8-bit characters.

For example, '그' is `0xadf8`, so would be written in a language file as `"\0xad\0xf8""`

## Docstrings

At the top of each language file should be a docstring for that file. This should have the form
```rust
//! Language file for {language}
//! 
//! Encoding: UTF-{8|16}
```
If the file is in development, you should add
```rust
//! Development version
```
to the bottom of the docstring. Before a release, all language files must be up-to-date and not be in active development. At release, only language files that are up-to-date and not being actively worked on will be included.
