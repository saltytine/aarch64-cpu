//! Holds the 64-bit virtual offset. This is the offset between the physical count value visible in
//! CNTPCT_EL0 and the virtual count value visible in CNTVCT_EL0.

use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "CNTVOFF_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "CNTVOFF_EL2", "x");
}

pub const CNTVOFF_EL2: Reg = Reg {};
