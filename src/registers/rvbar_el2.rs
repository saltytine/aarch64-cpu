//! If EL2 is the highest Exception level implemented, contains the
//! IMPLEMENTATION DEFINED address that execution starts from after reset when
//! executing in AArch64 state.

use tock_registers::interfaces::Readable;

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "RVBAR_EL2", "x");
}

pub const RVBAR_EL2: Reg = Reg;
