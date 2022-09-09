#!/bin/bash

# make 1MiB img file
dd if=/dev/zero of=disk.img count=256 bs=4k

# format as ext2 which has 1024 bytes for each block
mke2fs -b 1024 disk.img

