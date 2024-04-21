# tree-generator

<img src="./tree.png" height="400" />

This is a simple command line utility that generates random SVG trees from passed command-line arguments.

## Installation

To install this program check out the `releases` tab.

Additionally you may want to install this program from source. In that case:
- clone the repository onto your computer
- install rust toolchain
- execute `cargo build --release` command
The compiled version of the program should be placed in the `target/release` folder

## Usage

Since the tree-generator is in it's early stage of development, there are only few options it provides.
Those include:
- Max branch angle. It describes the maximum angle in which new branches may diverge
- Number of images to generate during the growth process
- Image prefix
- Time of growth