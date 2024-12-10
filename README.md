# arch-apt
An apt-like front-end for pacman written in Rust that uses verbs instead of flags for those switching to Arch Linux but having issues remembering some of the flags.

# Installation
To install, make sure you are on an Arch-based system that uses pacman as its package manager, and that you have Rust installed.
First, clone the repo: <br>
`$ git clone https://github.com/girof/arch-apt` <br>
Then, run: <br>
`$ cargo build -q -r --target-dir .` <br>
Then, you will have a pristine executable in a `release` folder in your current directory. Run it like that or put it somewhere on your PATH to use it without specifying the path. <br>
On Linux, run `echo $PATH` to see which directories are on your PATH.
<br><br>
That's all there is to do! have fun!
