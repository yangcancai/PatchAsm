# patch-core Feature
- Get base address
- Read memory online
- Write memory online

# patch-foo examle
- Remote thread inject for x64 in macos
## How to Use?

```shell
$ cargo build --release
$ cd patch-foo/csrc/ && make
$ ./add
res = 3, c=49, a=10, person.id=1, person.name=John Doe, addr = 0x0000600003f591e0
Library initialized!
pid: 24583, hex_address: 0x10bc6e008
ID: 100, Name: "hello"
Library loop running ...!
res = 3, c=49, a=10, person.id=100, person.name=hello, addr = 0x0000600003f591e0

$ sudo ./inject 24583 $(pwd)/../../target/release/libpatch_foo.dylib
Allocated remote stack @0x10bc9a000
pthread_create_from_mach_thread @7ff80999e1bb
dlopen @7ff8099a462f
Remote Stack 64  0x10bca2000, Remote code is 0x10bcaa000
Stub thread finished
```

