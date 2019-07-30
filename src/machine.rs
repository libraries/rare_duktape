use crate::{instruction_cycles, syscall};
use bytes::Bytes;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::{DefaultMachineBuilder, SupportMachine};
use std::cell::RefCell;
use std::rc::Rc;

pub enum MachineType {
    NativeRust,
    Asm,
}

pub fn exec(machine_type: MachineType, code: &Bytes, args: &[Bytes]) -> (i8, Vec<u8>, u64) {
    match machine_type {
        MachineType::NativeRust => {
            let ret_data = Rc::new(RefCell::new(Vec::new()));
            let mut machine = ckb_vm::DefaultMachineBuilder::<
                ckb_vm::DefaultCoreMachine<u64, ckb_vm::SparseMemory<u64>>,
            >::default()
            .instruction_cycle_func(Box::new(instruction_cycles))
            .syscall(Box::new(syscall::SyscallDebug::new(
                "log:",
                std::io::stdout(),
            )))
            .syscall(Box::new(syscall::SyscallRet::new(ret_data.clone())))
            .syscall(Box::new(syscall::SyscallStorage::new()))
            .syscall(Box::new(syscall::SyscallEmit::new()))
            .build();
            machine.load_program(code, args).unwrap();
            let exit = machine.run().unwrap();
            let cycles = machine.cycles();
            (exit, ret_data.clone().borrow().clone(), cycles)
        }
        MachineType::Asm => {
            let ret_data = Rc::new(RefCell::new(Vec::new()));
            let core_machine = AsmCoreMachine::new_with_max_cycles(1_000_000_000);
            let machine = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(core_machine)
                .instruction_cycle_func(Box::new(instruction_cycles))
                .syscall(Box::new(syscall::SyscallDebug::new(
                    "log:",
                    std::io::stdout(),
                )))
                .syscall(Box::new(syscall::SyscallRet::new(ret_data.clone())))
                .syscall(Box::new(syscall::SyscallStorage::new()))
                .syscall(Box::new(syscall::SyscallEmit::new()))
                .build();
            let mut machine = AsmMachine::new(machine);
            machine.load_program(code, args).unwrap();
            let exit = machine.run().unwrap();
            let cycles = machine.machine.cycles();
            (exit, ret_data.clone().borrow().clone(), cycles)
        }
    }
}
