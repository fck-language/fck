# fck Python Shell

This directory contains the source code for the fck shell and interpreter written in Python and released as v0.1.0-alpha. This directory has been removed from the repo as it is no longer maintained, but is still available through the release. This README file will detail what this release includes, and will walk you through installing and configuring fck. So to start, installing!

> This code is unmaintained! Please do not submit bug reports related to this code on the fck repo.  
> This code was written in Python 3.9, and forward compatibility is not guaranteed.

# Installing fck

1. Since this release is written in Python and cannot be compiled, you will have to have Python3+ installed (originally written in Python 3.9).  
2. After this you can choose where to put the source files, just make note of where you put them.  
3. To run fck, you need to run the ``cli.py`` file.  

That's it. All done. the next bit is an optional (but very much recommended) step to make using fck much easier. 

4. In your shell config file(``.zshrc`` for zsh or ``.bashrc`` for bash for example) add in an alias pointing to the ``cli.py`` file. For example ``alias fck="python3 path_to_Python_Shell_file/cli.py"`` for a ``.zshrc`` file  
That's it. fck is now installed as a Python shell and interpreted language on your system.

# Configuring fck

This release is a proof of concept and was written to be able to help in the development of fck-0.1.0, so configuration was not a main goal, thus only one option has been included for configuration. This is the wrap length of errors and warnings, which by default is set to 70. To change this, use the following line in your `.fck` config file, `wrapLength=` followed by an integer (N.B. this could technically be a floating point value but will be rounded). The wrap length has a minimum of 25, but no maximum.

## Where to put the config file

The config file should be placed in your root directory. If you're not sure where that is, open a terminal and type ``echo $HOME`` which should tell you where it is.

## Multi-lingual support

One of the main goals of fck was to create a language that you could use regardless of what your first language is, and to allow anyone to use code written by anyone from anywhere. This release does not include that.  
Creating a single coding language with multi-lingual support is incredibly complex and requires a lot of forethought to how certain aspects need to work, and for that to be embedded in the code from the beginning. If you require multi-lingual support, I invite you to take a look at the most recent release of [fck](https://github.com/fck-language/fck/releases) which does include multi-lingual support for all versions after this one. Each release comes with a README file which will include a list of the included languages in that version.

# Final words

I hope you enjoy fck, and thank you for giving it a try. As ever if you want to help develop it, you are more than welcome to. Unfortunately because the Python shell is no longer maintained after its release, any issues or bugs you find won't be able to be worked on as part of the project officially, but you have the source code, so feel free to play around and see what you can do.

I wish you the best of fun with this language

Rosie xx