use bytes::Bytes;
use rare_duktape::MachineType;
use std::fs;
use std::time::SystemTime;

const ITER: usize = 1000;

fn main() {
    let code = fs::read("./build/secp256k1_bench").unwrap();
    let args = vec![
        Bytes::from("main"),
        Bytes::from("033f8cf9c4d51a33206a6c1c6b27d2cc5129daa19dbd1fc148d395284f6b26411f"),
        Bytes::from("304402203679d909f43f073c7c1dcf8468a485090589079ee834e6eed92fea9b09b06a2402201e46f1075afa18f306715e7db87493e7b7e779569aa13c64ab3d09980b3560a3"),
        Bytes::from("foo"),
        Bytes::from("bar"),
    ];

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
