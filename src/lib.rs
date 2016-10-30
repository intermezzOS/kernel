#![feature(const_fn)]

// #![cfg_attr(not(test), no_std)]

#![no_std]

extern crate console;
extern crate interrupts;
extern crate spin;

pub mod kernel;
