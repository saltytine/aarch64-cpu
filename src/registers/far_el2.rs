//! Holds the faulting Virtual Address for all synchronous Instruction or Data Abort, PC alignment
//! fault and Watchpoint exceptions that are taken to EL2.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "FAR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "FAR_EL2", "x");
}

pub const FAR_EL2: Reg = Reg {};
