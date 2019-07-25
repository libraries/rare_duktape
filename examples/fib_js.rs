use bytes::Bytes;
use ckb_vm::machine::asm::{AsmCoreMachine, AsmMachine};
use ckb_vm::{DefaultMachineBuilder, SupportMachine};
use rare_duktape::{instruction_cycles, syscall};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::time::SystemTime;

const ITER: usize = 100;

fn main() {
    let code = fs::read("./build/duktape").unwrap();
    let jode = fs::read("./examples/fib.js").unwrap();
    let args = vec![Bytes::from("main"), Bytes::from(jode), Bytes::from("10")];

    let now = SystemTime::now();
    for i in 0..ITER {
        let ret_data = Rc::new(RefCell::new(Vec::new()));
        let core_machine = AsmCoreMachine::new_with_max_cycles(1_000_000_000);
        let machine = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(core_machine)
            .instruction_cycle_func(Box::new(instruction_cycles))
            .syscall(Box::new(syscall::SyscallDebug::new(
                "log:",
                std::io::stdout(),
            )))
            .syscall(Box::new(syscall::SyscallRet::new(ret_data.clone())))
            .build();
        let mut machine = AsmMachine::new(machine);

        machine
            .load_program(&Bytes::from(code.clone()), &args)
            .unwrap();

        let exit = machine.run().unwrap();
        let cycles = machine.machine.cycles();

        if i == 0 {
            println!(
                "First run result: exit={:?} ret={:?} cycles={:?}",
                exit,
                ret_data.borrow(),
                cycles
            );
        }
    }
    println!("Run {:?} in {:?}s", ITER, now.elapsed().unwrap().as_secs())
}
