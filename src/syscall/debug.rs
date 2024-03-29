//! Provedis a debug function, let the contract print information to standard output.
use std::io::Write;

use ckb_vm::instructions::Register;

use crate::syscall::common::get_str;
use crate::syscall::convention::SYSCODE_DEBUG;

pub struct SyscallDebug<T> {
    prefix: &'static str,
    output: T,
}

impl<T: Write> SyscallDebug<T> {
    pub fn new(prefix: &'static str, output: T) -> Self {
        Self { prefix, output }
    }
}

impl<Mac: ckb_vm::SupportMachine, T: Write> ckb_vm::Syscalls<Mac> for SyscallDebug<T> {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let code = &machine.registers()[ckb_vm::registers::A7];
        if code.to_i32() != SYSCODE_DEBUG {
            return Ok(false);
        }
        let addr = machine.registers()[ckb_vm::registers::A0].to_usize();
        let s = get_str(machine, addr)?;
        self.output.write_fmt(format_args!(
            "{} {} current_cycles={}\n",
            self.prefix,
            s,
            machine.cycles()
        ))?;
        machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(0));
        Ok(true)
    }
}
