//! Holds bits[63:0] of key A used for authentication of data pointer values.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "APDAKeyLo_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "APDAKeyLo_EL1", "x");
}

pub const APDAKEYLO_EL1: Reg = Reg {};
