//! When taking an exception to EL1, holds the address to return to.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "ELR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "ELR_EL1", "x");
}

pub const ELR_EL1: Reg = Reg {};
