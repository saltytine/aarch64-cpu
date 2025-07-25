//! Transfers data from the PE to an external debugger. For example, it is used by a debug target
//! to transfer data to the debugger. See DBGDTR_EL0 for additional architectural mappings. It is a
//! component of the Debug Communication Channel.

use tock_registers::interfaces::Writeable;

pub struct Reg;

impl Writeable for Reg {
    type T = u64;
    type R = ();

    sys_coproc_write_raw!(u64, "DBGDTRTX_EL0", "x");
}

pub const DBGDTRTX_EL0: Reg = Reg {};
