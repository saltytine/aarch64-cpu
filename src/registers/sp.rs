//! The stack pointer

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    read_raw!(u64, "sp", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    write_raw!(u64, "sp", "x");
}

pub const SP: Reg = Reg {};
