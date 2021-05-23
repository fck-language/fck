# fck Python Shell

This directory contains the source code for the fck shell written in Python and released as v0.1.0-alpha. This directory has been removed from the repo as it is no longer maintained, but is still available through the release. This README file will detail what this release includes, and will walk you through installing and configuring fck.

# Installing fck

1. Since this release is written in Python and cannot be compiled, you will have to have Python3+ installed (originally written in Python 3.8).  
2. After this you can choose where to put the source files, just make note of where you put them.  
3. To run fck, you need to run the ``cli.py`` file.  

That's it. All done. the next bit is an optional (but very much reccomended) step to make using fck much easier.  

4. In your shell config file(``.zshrc`` for zsh or ``.bashrc`` for bash for example) add in an alias pointing to the ``cli.py`` file. For example ``alias fck="python3 path_to_Python_Shell_file/cli.py"`` for a ``.zshrc`` file  
That's it. fck is now installed as a Python shell and interpereted language on your system.

# Configuring fck

This initial release doesn't include mayn configuration options as it was purely intended as a proof of concept, not a fully fledged language yet. However, there are two options that can be configured in the ``.fck`` config file, but first, making the file.

## Where to put the config file

The config file should be placed in your ``$HOME`` directory. If you're not sure where that is, open a terminal and type ``echo $HOME`` which should tell you where it is.

## 

