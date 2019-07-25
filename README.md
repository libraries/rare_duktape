# Rare Duktape

Tests for c/duktape on RISCV VM.

```
$ git clone https://github.com/mohanson/rare_duktape
$ cd rare_duktape
$ make riscv/duktape riscv/fib

$ cargo run --release --example fib_c
$ cargo run --release --example fib_js
```

# Test Result

```
# C
First run result: exit=0 ret=[0, 0, 0, 0, 0, 0, 0, 55] cycles=10431
Run 10000 in 5s, TPS=2000

# JavaScript
First run result: exit=0 ret=[0, 0, 0, 0, 0, 0, 0, 55] cycles=8064265
Run 100 in 20s, TPS=5
```

# Clean

```
$ docker rmi nervos/ckb-riscv-gnu-toolchain
```
