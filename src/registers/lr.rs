use tock_registers::interfaces::{Readable, Writeable};

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ();

    read_raw!(u64, "lr", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ();

    write_raw!(u64, "lr", "x");
}

pub const LR: Reg = Reg {};
