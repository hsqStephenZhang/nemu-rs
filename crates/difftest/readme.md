## how to use

1. in `ics-pa/nemu`, run `make menuconfig` to generate the headers (run other init commands before)
2. in `ics-pa/nemu/tools/spike-diff`, run `GUEST_ISA=riscv64 make -j$(nproc)` to build the spike (or GUEST_ISA=riscv32)
3. get the `ics-pa/nemu/tools/spike-diff/build/riscv64-spike-so` and pass it as the difftest dynamic lib

you can also try to modify the qemu-diff's source code and compile it for ISA=riscv64, and use it in the same way