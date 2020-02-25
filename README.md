ev3opt: An optimizer for LEGO Mindstorms EV3 bytecode
=====================================================

This project provides an optimizer for LEGO Mindstorms EV3 bytecode written
entirely in [Rust](https://www.rust-lang.org/).  It reads in EV3 bytecode,
runs a small suite of compiler optimization passes on it, and outputs it
again as EV3 bytecode.

## Status

Very very alpha.  I've demonstrated that it works and effectively optimizes
exactly one EV3 program.

## How to build

    # cargo build

## How to run

    # ev3opt input.rbf output.rbf

## How to get the .rbf file

Currently, there is no good way to do this.  How I'm doing it currently is
as follows:

 1. Write a program using either the iPad app or the desktop app
 2. Download the program to your EV3
 3. Insert a microSD card into the EV3 and transfer the program to it using
    the interface on the EV3 brick itself
 4. Use an SD card reader to transfer the file to the desktop or laptop
    computer where ev3opt is installed
 5. Run ev3opt on the computer
 6. Transfer the resulting file to the microSD card
 7. Plug the microSD card into the EV3 unit and run the program from the
    microSD card

Clearly, this is not an ideal solution.  I'm hoping once the project gets
better stabilized to come up with some sort of turn-key solution, ideally
one in which the optimizer runs on the EV3 brick itself.
