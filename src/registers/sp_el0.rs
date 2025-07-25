//! Holds the stack pointer associated with EL0. At higher Exception levels, this is used as the
//! current stack pointer when the value of SPSel.SP is 0.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "SP_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "SP_EL0", "x");
}

pub const SP_EL0: Reg = Reg {};
