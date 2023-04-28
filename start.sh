#! /bin/sh
#
# This script will be executed by `cargo run`.

set -xe

# Cargo passes the path to the built executable as the first argument.
KERNEL=$1

ARCH=$(echo "$KERNEL" | cut -d'/' -f2)
DEST_ISO_DIR="target/iso"
BOOT_DIR="boot"
GRUB_DIR="boot/grub"
GRUB_CONFIG_FILE="grub.cfg"
KERNEL_ISO="${KERNEL}.iso"

LOG_FILE="target/debug.log"
MEMORY_SIZE="4G"

# Copy the needed files into an ISO image.
mkdir -p "${DEST_ISO_DIR}/${GRUB_DIR}"
cp "${KERNEL}" "${DEST_ISO_DIR}/${BOOT_DIR}"
cp "${GRUB_DIR}/${GRUB_CONFIG_FILE}" "${DEST_ISO_DIR}/${GRUB_DIR}"

grub-mkrescue -o "${KERNEL_ISO}" "${DEST_ISO_DIR}"

# Run the created image with QEMU.
qemu-system-"${ARCH}" \
  -m "${MEMORY_SIZE}" \
  -drive file="${KERNEL_ISO}",format=raw \
  -no-reboot -no-shutdown \
  -D "${LOG_FILE}" \
  -d int \
  -serial stdio \
  -s