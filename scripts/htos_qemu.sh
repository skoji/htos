#!/bin/bash -ex

DISKIMG=disk.img
MOUNTPOINT=mnt
BOOTLOADER=BOOTX64.EFI
KERNEL=target/x86_64-unknown-linux-gnu/debug/htos

# ディスクイメージ作成
rm -f $DISKIMG
qemu-img create -f raw $DISKIMG 200M
mkfs.fat -n 'HTOS' -s 2 -f 2 -R 32 -F 32 $DISKIMG

# マウントポイント作成
mkdir -p $MOUNTPOINT
sudo mount -o loop $DISKIMG $MOUNTPOINT

# ディスクの中身作成
sudo mkdir -p $MOUNTPOINT/EFI/BOOT
sudo cp $BOOTLOADER $MOUNTPOINT/EFI/BOOT/BOOTX64.EFI
sudo cp $KERNEL $MOUNTPOINT/htos.elf

# アンマウント
sleep 0.5
sudo umount $MOUNTPOINT

# QEMU起動
sudo qemu-system-x86_64 \
    -m 1G \
    -drive if=pflash,format=raw,readonly,file=/usr/share/OVMF/OVMF_CODE.fd \
    -drive if=pflash,format=raw,file=/usr/share/OVMF/OVMF_VARS.fd \
    -drive if=ide,index=0,media=disk,format=raw,file=$DISKIMG \
    -monitor stdio \
    $QEMU_OPTS
