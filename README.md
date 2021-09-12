<p align="center">
    <img src="/img/logo/logo_2.png" alt="fck readme header image" width=90%>
</p>

![build](https://github.com/fck-language/fck/actions/workflows/rust_build.yml/badge.svg)
[![wakatime](https://wakatime.com/badge/github/fck-language/fck.svg)](https://wakatime.com/badge/github/fck-language/fck)
![lines](https://img.shields.io/tokei/lines/github/fck-language/fck?color=%23ff4a69)
![license](https://img.shields.io/github/license/fck-language/fck)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](CODE_OF_CONDUCT.md)

fck is the worlds first coding language with full support for any language imaginable! Seems fancy right?! We have designed the language to be able to seamlessly switch between languages, so you can work on code with anyone from anywhere! fck is also a compiled and interpreted language, allowing complete flexibility in your workflow.

---

## Contents

- [Installing](#installing)
  - [SHA sums](#sha-sums)
  - [Flavours](#flavours)
- [Syntax highlighting](#syntax-highlighting)
- [Contributing](#contributing)

---

# Installing

Installing fck is fairly simple. Currently, you need to go the the [releases page](https://github.com/fck-language/fck/releases) and pick your version and [flavour](#flavours).
Once the file is downloaded, we highly recommend you check the SHA sums of the file against this readme. If the SHA sums match up, then uncompress the file and cd into the uncompressed directory. Then run `install.sh` with `sudo` permissions. This will install the correct binary file for your system.

## SHA sums

<table>
    <thead>
        <tr>
            <th>File</th>
            <th>Algorithm</th>
            <th>SHA sum</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan=2><a href=https://github.com/RosiePuddles/fck/releases>fck-v0_1_0.tar.gz</a></td>
            <td>256</td>
            <td> </td>
        </tr>
        <tr>
            <td>512</td>
            <td> </td>
        </tr>
    </tbody>
</table>

## Flavours

As well as fck being the first language to support multiple languages, it is also the first (so far as I can tell) language that implements *flavours*. Each release will have several flavours, each with its own special additions and each being suited for different types of programmers. For a more in depth explanation and the features of each flavour, see the flavours section of the fck website. The current flavours are as follows:

### Pure

This flavour is the simplest flavour, and is what all other flavours are built off of. Everything in the pure flavour is in all other flavours.

### Counting

A maths based flavour with added constants and built-in functions

#### Constants

- `phi`, The golden ratio (≈1.61803)
- `euler_mascheroni`, Euler-Mascheroni constant (≈0.57722)
- `catalan`, Catalan's constant (≈0.91597)
- `wierstrass`, Wierstrass' constant (≈0.47495)

and others...

#### Functions

- `gamma`, Gamma function
- `digamma`, Digamma function
- `beta`, Beta function
- `polylogarithm`, Polylogarithm functions

and others...

---

# Syntax highlighting

fck has separate packages available for [Atom](https://atom.io), [Vim](https://www.vim.org), and [VS Code](https://code.visualstudio.com) for syntax highlighting. Please be aware that these are not maintained at the same rate as the source code, and often lag behind development. These are often left until just before the next release to be updated.

- [Atom highlighting](https://github.com/RosiePuddles/language-fck)
- [Vim highlighting](https://github.com/fck-language/fck.vim)
- [VS Code highlighting]()

# Contributing

Contribution is always welcome! Before contributing, please make sure you have a read of our [contributing guidelines](CONTRIBUTING.md). If you have any questions after reading that, feel free to ask them on the [Q&A section](https://github.com/fck-language/fck/discussions/categories/q-a) of our discussions page.

[comment]: <> (fck has, in a bit, been benchmarked using [The Computer Language Benchmarks Game]&#40;https://benchmarksgame-team.pages.debian.net/benchmarksgame/&#41;. At current, only times are given for execution, all of which are given relative to fck-interpreted:)
