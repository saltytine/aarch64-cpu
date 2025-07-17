//! Holds bits[127:64] of key B used for authentication of instruction pointer values.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "APIBKeyHi_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "APIBKeyHi_EL1", "x");
}

pub const APIBKEYHI_EL1: Reg = Reg {};
