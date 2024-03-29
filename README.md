<p align="center">
<img src="/img/logo/logo_2.png" alt="fck readme header image" width=90%>
</p>

<div align="center">

![build](https://img.shields.io/github/workflow/status/fck-language/fck/build?style=flat-square&logo=github)
![license](https://img.shields.io/github/license/fck-language/fck?style=flat-square)
![build](https://img.shields.io/github/repo-size/fck-language/fck?style=flat-square&logo=github)
</div>

<h4>
fck is the first coding language built around collaboration; allowing you to write code in any human language you want with no compromises.
</h4>

---

## Contents

- [Installing](#installing)
  - [Dependencies](#dependencies)
- [Flavours](#flavours)
- [Things of note](#things-of-note)
- [Syntax highlighting](#syntax-highlighting)
- [External Dependencies](#external-dependencies)
- [Contributing](#contributing)

---

# Installing

Please read [the installation guide](INSTALLING.md).

## Dependencies

fck depends upon clang for compiling object files. If you do not have clang installed, we highly recommend you do so; not only for fck but for a vast number of other projects. Without clang, you will not be able to compile code into executable files, and will only be able to make use of the JIT compiler (interpreter) that uses plain LLVM.

# Flavours

As well as fck being the first language to support multiple languages, it is also the first (so far as I can tell) language that implements flavours. Each release will have several flavours, each with its own special additions and each being suited for different types of programmers. For a more in depth explanation and the features of each flavour, see the flavours section of the [fck website](https://rosiepuddles.github.io/fck/). The current flavours are as follows:
- Pure

Flavours to be added later: 
- Counting

# Things of note

fck has a few interesting quirks as a consequence of a few design choices. These shouldn't impact you in general, but they are worth being aware of.
1. **Interpreting code does not run section by section**\
Interpreted coding languages, because they're interpreted, can run code as it reads it, allowing errors to be far down the code and not impact anything before it. Because fck is both an interpreted and compiled language, the ASTs are generated for the entire file all at once. In general this will not impact you, and may even be more useful since it will pick up on errors before the code is run.
2. **Multilingual support is further reaching than you thought**\
Because fck is multilingual, adding a new feature has to be considered in every language currently implemented, and then documentation is needed for all the different languages. This also means that testing features takes longer that normal because all languages have to be tested.
3. **Debug mode is only in English**\
Debug mode is only intended to be used by people developing the language. Because Rust is in English, there isn't much point in extending multilingual support to debug. If you really do want it to be extended, that can be done though

---

# Syntax highlighting

fck has separate packages available for syntax highlighting for several major IDEs/text editors. Please be aware that these are not maintained at the same rate as the source code, and often lag behind development. These are often left until just before the next release to be updated.

- [Vim](https://github.com/fck-language/fck.vim)
- [Atom](https://github.com/RosiePuddles/language-fck) ***Not started***
- [Sublime]() ***Not started***
- [VS Code]() ***Not started***
- [Visual Studio]() ***Not started***
- [Notepad++]() ***Not started***

# External Dependencies

fck has a few external dependencies, but we try to limit this. Currently, we rely upon a total of 6 crates (5 for fck proper and 1 for the type system). These are explained below:

| Crate                                                   | Version | What we use it for                                                                                                                                                      |
|---------------------------------------------------------|---------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [`llvm-sys`](https://crates.io/crates/llvm-sys/130.0.3) | 130.0.3 | This gives us bindings to LLVM and is the basis of the compilers                                                                                                        |
| [`colored`](https://crates.io/crates/colored/2.0.0)     | 2.0.0   | Formatting terminal text is different for different platforms, so we let `colored` take care of it for us                                                               |
| [`git2`](https://crates.io/crates/git2/0.13.25)         | 0.13.25 | For project git initialisation                                                                                                                                          |
| [`dirs`](https://crates.io/crates/dirs/4.0.0)           | 4.0.0   | This gives us system dependent directory locations such as the home directory and application storage location                                                          |
| [`reqwest`](https://crates.io/crates/reqwest/0.11.10)   | 0.11.10 | We use this when translating files. If the `--comment` flag is present, then comments are translated using LibreTranslate. `reqwest` handles the POST requests for us   |
| [`phf`](https://crates.io/crates/phf/0.10.1)            | 0.10.1  | Used for having hash map like tables as constants. Required so that the built-in types can be constants, and don't have to be returned from functions (speed basically) |

This project also uses [`clap`](https://crates.io/crates/clap/3.1.15) version 3.1.15 through the [`lang`](https://github.com/fck-language/lang) dependency for command line argument parsing

# Contributing

Contribution is always welcome! Before contributing, please make sure you have a read of our [contributing guidelines](CONTRIBUTING.md). If you have any questions after reading that, feel free to ask them on the [Q&A section](https://github.com/fck-language/fck/discussions/categories/q-a) of our discussions page.
