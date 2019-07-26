# Rare Duktape

Tests for c / duktape on RISCV VM

```
$ git clone https://github.com/mohanson/rare_duktape
$ cd rare_duktape
$ make riscv/duktape riscv/fib

$ cargo run --release --example fib_c
$ cargo run --release --example fib_js
```

# Test Result

Test fibonacci by C and JS, on Asm machine and native rust interpter:

```
# C
Use asm machine, 10000 times
First run result: exit=0 ret=[0, 0, 0, 0, 0, 0, 0, 55] cycles=10431
Run 10000 in 5s, TPS=2000

Use native rust, 10000 times
First run result: exit=0 ret=[0, 0, 0, 0, 0, 0, 0, 55] cycles=10431
Run 10000 in 4s, TPS=2500

# JavaScript
Use asm machine, 100 times
First run result: exit=0 ret=[0, 0, 0, 0, 0, 0, 0, 55] cycles=8064265
Run 100 in 21s, TPS=4

Use native rust, 100 times
First run result: exit=0 ret=[0, 0, 0, 0, 0, 0, 0, 55] cycles=8064265
Run 100 in 32s, TPS=3
```

Test secp256k1 on Asm machine and native rust interpter, source code are taken from [here](https://github.com/nervosnetwork/ckb-vm-test-suite):

```
Use asm machine, 1000 times
First run result: exit=0 ret=[] cycles=1330941
Run 1000 in 5s, TPS=200

Use native rust, 1000 times
First run result: exit=0 ret=[] cycles=1330941
Run 1000 in 54s, TPS=18
```

# Clean

```
$ docker rmi nervos/ckb-riscv-gnu-toolchain
$ rm -rf $(PROJECT_DIR)
```
