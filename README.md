# arch-apt
An apt-like front-end for pacman written in Rust that uses verbs instead of flags for those switching to Arch Linux but having issues remembering some of the flags.

# Installation
To install, make sure you are on an Arch-based system that uses pacman as its package manager, and that you have Rust installed.
First, clone the repo:
`$ git clone https://github.com/girof/arch-apt`
Then, run:
`$ cargo build -q -r --target-dir .`
Then, you will have a pristine executable in a `release` folder in your current directory. Run it like that or put it somewhere on your PATH to use it without specifying the path.
On Linux, run `echo $PATH` to see which directories are on your PATH.
