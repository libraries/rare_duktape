use bytes::Bytes;
use rare_duktape::MachineType;
use std::fs;
use std::time::SystemTime;

const ITER: usize = 100;

fn main() {
    let code = fs::read("./build/duktape").unwrap();
    let jode = fs::read("./examples/fib.js").unwrap();
    let args = vec![Bytes::from("main"), Bytes::from(jode), Bytes::from("10")];

    println!("Use asm machine, {:?} times", ITER);
    let now = SystemTime::now();
    for i in 0..ITER {
        let (exit, ret_data, cycles) =
            rare_duktape::exec(MachineType::Asm, &Bytes::from(code.clone()), &args);
        if i == 0 {
            println!(
                "First run result: exit={:?} ret={:?} cycles={:?}",
                exit, ret_data, cycles
            );
        }
    }
    let d = now.elapsed().unwrap().as_secs();
    println!("Run {:?} in {:?}s, TPS={:?}", ITER, d, ITER / d as usize);
    println!("");

    println!("Use native rust, {:?} times", ITER);
    let now = SystemTime::now();
    for i in 0..ITER {
        let (exit, ret_data, cycles) =
            rare_duktape::exec(MachineType::NativeRust, &Bytes::from(code.clone()), &args);
        if i == 0 {
            println!(
                "First run result: exit={:?} ret={:?} cycles={:?}",
                exit, ret_data, cycles
            );
        }
    }
    let d = now.elapsed().unwrap().as_secs();
    println!("Run {:?} in {:?}s, TPS={:?}", ITER, d, ITER / d as usize);
}
