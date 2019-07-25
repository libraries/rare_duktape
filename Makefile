TARGET := riscv64-unknown-elf
CC := $(TARGET)-gcc
LD := $(TARGET)-gcc
CFLAGS := -Os -DCKB_NO_MMU -D__riscv_soft_float -D__riscv_float_abi_soft
APP_CFLAGS := $(CFLAGS) -Isrc/duktape -Ic -Wall -Werror
LDFLAGS := -lm -Wl,-static -fdata-sections -ffunction-sections -Wl,--gc-sections -Wl,-s
CURRENT_DIR := $(shell pwd)
DOCKER_BUILD := docker run -v $(CURRENT_DIR):/src nervos/ckb-riscv-gnu-toolchain:bionic bash -c

riscv/duktape/real:
	$(CC) $(APP_CFLAGS) src/c/entry.c -c -o build/entry.o
	$(CC) $(APP_CFLAGS) src/duktape/duktape.c -c -o build/duktape.o
	$(LD) build/entry.o build/duktape.o -o build/duktape $(LDFLAGS)

riscv/duktape: riscv/duktape/real
	$(DOCKER_BUILD) "cd /src && make riscv/duktape/real"

riscv/fib/real:
	$(CC) -I./src/c/ -o ./build/fib ./examples/fib.c

riscv/fib: riscv/fib/real
	$(DOCKER_BUILD) "cd /src && make riscv/fib/real"
