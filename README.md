# libopencm3_sys

This is a project containing Rust bindings for the libopencm3 library, which is a firmware library for ARM Cortex-M3 microcontrollers and some other Cortex-M micros.

libopencm3 is available at <https://github.com/libopencm3/libopencm3>.


## Status

These bindings currently only provide functions belonging to STM32F1-family microcontrollers.

The build script and `wrapper.h` would need to be changed to accommodate other microcontrollers -- a fork could easily do this until I figure out how to do it within the one repository.


### libopencm3

libopencm3 itself is reportedly in flux.


## Usage

I don't yet have any usage example for this as I have not yet figured it out myself.


## Licence

This repository is under the LGPL-3 licence, as per libopencm3.

This repository contains some code adapted from the Rust `std` library -- where noted, that code is under the same licence as the original Rust `std` library.


## Contributions

Contributions are welcomed.

