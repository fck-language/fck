<p align="center">
    <img src="/img/logo/icon.jpeg" alt="fck readme header image">
</p>

# fck  
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](code_of_conduct.md)

fck is an interpereted and compiled coding language, for fast and easy debugging, and fast binary files. fck has been designed in such a way so as to make it very difficult to break, with an emphasis placed on non-breaking warnings and trying to fix any small errors in the code.

fck is completely open source and written in C++, with extensive [documentation](https://RosiePuddles.github.io/fck/docs) and [tutorials](https://rosiepuddles.github.io/fck/tutorial) to get you started on the language.

If English isn't your first language, then don't worry! fck has multilingual support, which means that you can change the language of the syntax is in at any point in the code.

Okay have a fun day darling xx

# Installing

To install a version of fck, download one of the .tar.gz files from the [releases page](https://github.com/RosiePuddles/fck/releases). All the SHA sums are given below, and each release hs its SHA sums along with the release. If you download a release and it doesn't have the correct SHA sums, redownload the file and check again. If it still doesn't, check the discussions page for that release, linked with the release, and see if anyone else has had the same problem. If no one has, ask and we'll have a look. If the SHA sums need changing, they will be done so as soon as possible.

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
<!--         <tr>
            <td rowspan=2><a href=release_link>release .tar.gz file</a></td>
            <td>256</td>
            <td>SHA256sum</td>
        </tr>
        <tr>
            <td>512</td>
            <td>SHA512sum</td>
        </tr> -->
        <tr>
            <td rowspan=2><a href=https://github.com/RosiePuddles/fck/releases>fck-v0_1_0-alpha.tar.gz</a></td>
            <td>256</td>
            <td> </td>
        </tr>
        <tr>
            <td>512</td>
            <td> </td>
        </tr>
    </tbody>
</table>

# Syntax highlighting

fck has seperate packages available for [Atom](https://atom.io) and [Vim](https://www.vim.org) for syntax highlighting. Please be aware that these are not maintained at the same rate as fck, and often lag behind development to make sure that any changes or additions made to fck are final before being added into the syntax highlighting. That being said, before any release these are updated to be in line with the upcoming release.

- [Atom highlighting](https://github.com/RosiePuddles/language-fck)
- [Vim highlighting]()

# Benchmarking

fck has been (in a bit) benchmarked using [The Computer Language Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/). At current, only times are given for execution, all of which are given relative to fck-interpreted:

<table>
    <thead>
        <tr>
            <td rowspan="2">Language</td>
            <td colspan="10" align="center">Benchmark</td>
        </tr>
        <tr>
            <td>fannkuch-redux</td>
            <td>n-body</td>
            <td>spectral-norm</td>
            <td>mandlebrot</td>
            <td>pidigits</td>
            <td>regex-redux</td>
            <td>fasta</td>
            <td>k-nucleotide</td>
            <td>reverse-complement</td>
            <td>binary-trees</td>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>fck-0.1.0-alpha</td>
            <td>1</td>
            <td>1</td>
            <td>1</td>
            <td>1</td>
            <td>1</td>
        </tr>
        <tr>
            <td>CPython-3.2</td>
            <td>352.29</td>
            <td>567.56</td>
            <td>120.99</td>
            <td>13.8</td>
            <td>5.1</td>
        </tr>
        <tr>
            <td>GCC-4.3.2(C)</td>
            <td>1.0</td>
            <td>2.3</td>
            <td>1.7</td>
            <td>4.5</td>
            <td>3.0</td>
        </tr>
        <tr>
            <td>Java@JRE-1.6.0_25</td>
            <td>1.7</td>
            <td>2.6</td>
            <td>6.8</td>
            <td>13.4</td>
            <td>6.7</td>
        </tr>
        <tr>
            <td>Ruby-1.9.2p180</td>
            <td>98.0</td>
            <td>628.4</td>
            <td>15.4</td>
            <td>30.3</td>
            <td>8.6</td>
        </tr>
    </tbody>
</table>
