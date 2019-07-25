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
First run result: exit=0 cycles=10431 ret=[0, 0, 0, 0, 0, 0, 0, 55]
Run 10000 in 5s

# JavaScript
First run result: exit=0 cycles=8064265 ret=[0, 0, 0, 0, 0, 0, 0, 55]
Run 100 in 21s
```
