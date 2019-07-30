function main(argc, argv) {
    // 5 次系统调用
    pvm.debug('Hello');
    pvm.debug('Hello');
    pvm.debug('Hello');
    pvm.debug('Hello');
    pvm.debug('Hello');

    // 7 次世界状态 set
    var buffer_k = new Buffer([0x65, 0x6e, 0x76, 0x2e, 0x6b])
    var buffer_v = new Buffer([0x65, 0x6e, 0x76, 0x2e, 0x76])
    pvm.save(buffer_k, buffer_v)
    pvm.save(buffer_k, buffer_v)
    pvm.save(buffer_k, buffer_v)
    pvm.save(buffer_k, buffer_v)
    pvm.save(buffer_k, buffer_v)
    pvm.save(buffer_k, buffer_v)
    pvm.save(buffer_k, buffer_v)

    // 8 次世界状态 get
    pvm.load(buffer_k)
    pvm.load(buffer_k)
    pvm.load(buffer_k)
    pvm.load(buffer_k)
    pvm.load(buffer_k)
    pvm.load(buffer_k)
    pvm.load(buffer_k)
    pvm.load(buffer_k)

    // 7 次加法
    var a = 0;
    a += 10;
    a += 10;
    a += 10;
    a += 10;
    a += 10;
    a += 10;
    a += 10;

    // 3 次减法
    a -= 10;
    a -= 10;
    a -= 10;

    // 1 次取反
    var b = false;
    b = !b;

    // 6 次乘法
    a *= 10;
    a *= 10;
    a *= 10;
    a *= 10;
    a *= 10;
    a *= 10;

    // 10 次除法
    a /= 2;
    a /= 2;
    a /= 2;
    a /= 2;
    a /= 2;
    a /= 2;
    a /= 2;
    a /= 2;
    a /= 2;
    a /= 2;

    // 6 次或
    c = true || false;
    c = true || false;
    c = true || false;
    c = true || false;
    c = true || false;
    c = true || false;

    // 9 次和
    c = true && false;
    c = true && false;
    c = true && false;
    c = true && false;
    c = true && false;
    c = true && false;
    c = true && false;
    c = true && false;
    c = true && false;

    // 31 次比较
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
    d = a > 10;
}
