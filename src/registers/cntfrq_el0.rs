//! This register is provided so that software can discover the frequency of the system counter. It
//! must be programmed with this value as part of system initialization. The value of the register
//! is not interpreted by hardware.

use tock_registers::interfaces::Readable;

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_read_raw!(u64, "CNTFRQ_EL0", "x");
}

pub const CNTFRQ_EL0: Reg = Reg {};
