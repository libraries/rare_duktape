function getInt64Bytes(x) {
    buffer = new Buffer(8)
    buffer[0] = (x / 2 ** 56) & 0xFF
    buffer[1] = (x / 2 ** 48) & 0xFF
    buffer[2] = (x / 2 ** 40) & 0xFF
    buffer[3] = (x / 2 ** 32) & 0xFF
    buffer[4] = (x / 2 ** 24) & 0xFF
    buffer[5] = (x / 2 ** 16) & 0xFF
    buffer[6] = (x / 2 **8) & 0xFF
    buffer[7] = x & 0xFF
    return buffer
}

function fib(n) {
  if (n == 0 || n == 1) {
      return n
  } else {
      return (fib(n-1) + fib(n-2))
  }
}

function main(argc, argv) {
    var n = parseInt(argv[1], 10)
    var r = fib(n)
    pvm.ret(getInt64Bytes(r))
}
