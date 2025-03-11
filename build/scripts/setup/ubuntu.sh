#! bin/bash

# Update package repository
apt-get update

# Install build-essential by default.
apt-get install -y build-essential

apt-get install -y  \
    grub2           \
    ld              \
    nasm            \
    qemu            \
