//! Provedis a debug function, let the contract print information to standard output.
use ckb_vm::instructions::Register;

#[derive(Default)]
pub struct SyscallEmit {}

impl SyscallEmit {
    pub fn new() -> Self {
        Self {}
    }
}

impl<Mac: ckb_vm::SupportMachine> ckb_vm::Syscalls<Mac> for SyscallEmit {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }

    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let code = &machine.registers()[ckb_vm::registers::A7];
        if code.to_i32() == 0x1010_1010 {
            machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u8(0));
            return Ok(true);
        }
        Ok(false)
    }
}
