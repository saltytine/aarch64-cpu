//! The frame pointer register

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    read_raw!(u64, "x29", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    write_raw!(u64, "x29", "x");
}

pub const FP: Reg = Reg {};
