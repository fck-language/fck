# fck Python Shell

This directory contains the source code for the fck shell and interpreter written in Python and released as v0.1.0-alpha. This directory has been removed from the repo as it is no longer maintained, but is still available through the release. This README file will detail what this release includes, and will walk you through installing and configuring fck. So to start, installing!

# Installing fck

1. Since this release is written in Python and cannot be compiled, you will have to have Python3+ installed (originally written in Python 3.8).  
2. After this you can choose where to put the source files, just make note of where you put them.  
3. To run fck, you need to run the ``cli.py`` file.  

That's it. All done. the next bit is an optional (but very much recommended) step to make using fck much easier.  

4. In your shell config file(``.zshrc`` for zsh or ``.bashrc`` for bash for example) add in an alias pointing to the ``cli.py`` file. For example ``alias fck="python3 path_to_Python_Shell_file/cli.py"`` for a ``.zshrc`` file  

That's it. fck is now installed as a Python shell and interpreted language on your system.

# What does this release include?

This release of fck includes some fairly basic things, variables (``int``, ``float``, ``bool``, ``str``, and ``list``), [functions](#functions), [built-in methods](#built-in-methods), amongst other stuff that's fairly obvious like arithmetic operators

## Functions

Functions are defined using the ``def`` keyword followed by an optional name then arguments in brackets with optional default values. The arguments do not have to be ordered by whether it has a default value or not. After the arguments you can add an optional return type. The suite is then enclosed in the typical curly brackets, meaning no indentation is required, it's just a nice thing to have.  
For more information on functions, see the [documentation](https://rosiepuddles.github.io/fck/docs/_build/html/Functions) page

## Built-in Methods

This is a list of all the implemented built-in methods and a brief description of them:

- ``print``  
  Prints a value to the console
- ``log``  
  Prints a value to the console with no newline at the end. Can be disabled globally
- ``type``  
  Returns the type of the value passed in as a ``str``
- ``as``  
  Casts the given value as a different given type
- ``iterate``  
  Iterates over a given range with an optionally given step, or iterates over a list
- ``while``  
  Runs a suite while a condition evaluates to ``True``
- ``if``, ``else``, and ``elif``  
  Runs a suite if a condition is ``True``, otherwise checks the conditions for all related ``elif`` statements, and if none evaluate to ``True`` the ``else`` statement suite will be executed if any is given
- ``case``, ``option``, and ``default``  
  ``case`` takes a given value and checks this against all the given vales for all the ``option`` statements, and if none match will run the ``default`` suite if one is given
- ``?``  
  In-line if else statement. Can also be used to catch errors in type casting

This list is exceptionally condensed for the purposes of this README file to give a general overview of the methods. For more information, and better explanations, on built-in methods, see the [documentation](https://rosiepuddles.github.io/fck/docs/_build/html/Functions) page

# Configuring fck

This initial release doesn't include many configuration options as it was purely intended as a proof of concept, not a fully fledged language yet. However, there are two options that can be configured in the ``.fck`` config file, but first, making the file.

## Where to put the config file

The config file should be placed in your ``$HOME`` directory. If you're not sure where that is, open a terminal and type ``echo $HOME`` which should tell you where it is.

## Configurable options

- ``wrapLength``
- ``something else``

## Errors and warnings

As with all of fck, the errors and warnings have custom error messages. This is explained in greater depth in the [docs](https://rosiepuddles.github.io/fck/docs/_build/html/Error%20messages.html). The file for the error messages should be placed in the same directory as the source code and called ``errors.txt``

# Final words

I hope you enjoy fck and thank you for giving it a try. As ever if you want to help develop it, you are more than welcome to. Unfortunately, because the Python shell is no longer maintained after its release, any issues or bugs you find won't be able to be worked on as part of the project officially, but you have the source code, so feel free to play around and see what you can do.  
If you do decide to play around with the source code, I sincerely apologise for the lack of comments in it. Sorry

I with you the bestest of fun with this language darling, and hope you install the full maintained version xx

