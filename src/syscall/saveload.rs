// Provides the ability to interact with state storage.
use ckb_vm::instructions::Register;

use crate::syscall::convention::{SYSCODE_LOAD, SYSCODE_SAVE};

pub struct SyscallStorage {}

impl SyscallStorage {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Mac: ckb_vm::SupportMachine> ckb_vm::Syscalls<Mac> for SyscallStorage {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let code = &machine.registers()[ckb_vm::registers::A7];

        // Set storage
        if code.to_i32() == SYSCODE_SAVE {
            machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(0));
            return Ok(true);
        }

        // Get storage
        if code.to_i32() == SYSCODE_LOAD {
            machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(0));
            return Ok(true);
        }
        Ok(false)
    }
}
