#![feature(const_fn)]

#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate core;

extern crate console;
extern crate interrupts;
extern crate spin;

pub mod kernel;
