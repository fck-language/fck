<p align="center">
<img src="/img/logo/logo_2.png" alt="fck readme header image" width=90%>
</p>

![build](https://github.com/fck-language/fck/actions/workflows/rust_build.yml/badge.svg)
[![wakatime](https://wakatime.com/badge/github/fck-language/fck.svg)](https://wakatime.com/badge/github/fck-language/fck)
![lines](https://img.shields.io/tokei/lines/github/fck-language/fck?color=%23ff4a69)
![license](https://img.shields.io/github/license/fck-language/fck)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](CODE_OF_CONDUCT.md)

fck is the worlds first coding language with full support for any human language! Seems fancy right?! We have designed the language to be able to seamlessly switch between languages, so you can work on code with anyone from anywhere! fck is also a compiled and interpreted language, allowing complete flexibility in your workflow.

---

## Contents

- [Installing](#installing)
- [Flavours](#flavours)
- [Things of note](#things-of-note)
- [Syntax highlighting](#syntax-highlighting)
- [Contributing](#contributing)

---

# Installing

Installing fck is fairly simple. Currently, you need to go the the [releases page](https://github.com/fck-language/fck/releases) and pick your version and [flavour](#flavours).
Once the file is downloaded, we highly recommend you check the SHA sums of the file against the ones in the release post. If the SHA sums match up, then uncompress the file and cd into the uncompressed directory. Then run `install.sh` with `sudo` permissions. This will install the correct binary file for your system.

# Flavours

As well as fck being the first language to support multiple languages, it is also the first (so far as I can tell) language that implements flavours. Each release will have several flavours, each with its own special additions and each being suited for different types of programmers. For a more in depth explanation and the features of each flavour, see the flavours section of the [fck website](https://rosiepuddles.github.io/fck/). The current flavours are as follows:
- Pure
- Counting

# Things of note

fck has a few interesting quirks as a consequence of a few design choices. These shouldn't impact you in general, but they are worth being aware of.
1. **Interpreting code does not run section by section**
Interpreted coding languages, because they're interpreted, can run code as it reads it, allowing errors to be far down the code and not impact anything before it. Because fck is both an interpreted and compiled language, the ASTs are generated for the entire file all at once. In general this will not impact you, and may even be more useful since it will pick up on errors before the code is run.
2. **Multi-lingual support is further reaching than you thought**
Because fck is multi-lingual, adding a new feature has to be considered in every language currently implemented, and then documentation is needed for all the different languages. This also means that testing features takes longer that normal because all languages have to be tested.

---

# Syntax highlighting

fck has separate packages available for syntax highlighting for several major IDEs/text editors. Please be aware that these are not maintained at the same rate as the source code, and often lag behind development. These are often left until just before the next release to be updated.

- [Vim](https://github.com/fck-language/fck.vim)
- [Atom](https://github.com/RosiePuddles/language-fck) ***Not started***
- [Sublime]() ***Not started***
- [VS Code]() ***Not started***
- [Visual Studio]() ***Not started***
- [Notepad++]() ***Not started***

# Contributing

Contribution is always welcome! Before contributing, please make sure you have a read of our [contributing guidelines](CONTRIBUTING.md). If you have any questions after reading that, feel free to ask them on the [Q&A section](https://github.com/fck-language/fck/discussions/categories/q-a) of our discussions page.
