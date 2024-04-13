# hhuTOSr

## Requirements

For building hhuTOSr, a _rust nightly_ toolchain is needed. To install _rust_ use [rustup](https://rustup.rs/).
Deb-Based: `apt-get install qemu-system`

`rustup toolchain install nightly`

And activate it for the current folder with:

`rustup override set nightly`

To run the build commands _cargo-make_ is required. Install it with:

`cargo install --no-default-features cargo-make`

Further the following packages for Debian/Ubuntu based systems (or their equivalent packages on other distributions) need to be installed:

`sudo apt install build-essential nasm mtools fdisk zstd`

To run the final OS image _QEMU_ is required:

(`apt install qemu-system-x86_64`)
`sudo apt-get install qemu-system` funktioniert besser

### Eigene Zusatzschritte

Component hinzufügen: `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`

Xorriso installieren: 
* `sudo apt-add-repository universe`
* `sudo apt-get install xorriso`

## Build

For a full build run: 

`cargo make`


## Run

To run the image, build it first and then use:

`cargo make qemu`

This will execute the operating system with _QEMU_.
