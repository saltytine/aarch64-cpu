//! Holds the 64-bit virtual count value. The virtual count value is equal to the physical count
//! value in `CNTPCT_EL0` minus the virtual offset visible in `CNTVOFF_EL2`

use tock_registers::interfaces::Readable;

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "CNTVCT_EL0", "x");
}

pub const CNTVCT_EL0: Reg = Reg {};
