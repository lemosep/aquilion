# Usage

Build with `cargo build`, then run the qemu command:


```
qemu-system-x86_64 -enable-kvm \
-drive if=pflash,format=raw,readonly=on,file=./ovmf/OVMF_CODE_4M.fd \
-drive if=pflash,format=raw,readonly=on,file=./ovmf/OVMF_VARS_4M.fd \
-drive format=raw,file=fat:rw:esp