#![feature(asm, concat_idents, const_fn, const_cell_new, try_from)]
#![no_std]
#![crate_name = "cc26xx"]
#![crate_type = "rlib"]
extern crate bitfield;
#[allow(unused_imports)]
#[macro_use(debug)]
extern crate kernel;

pub mod aon;
pub mod gpio;
pub mod ioc;
pub mod prcm;
pub mod ccfg;
pub mod peripheral_interrupts;
