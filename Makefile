# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

override IMAGE_NAME := hikari
override BUILD_DIR := build

.SHELLFLAGS += -e

.PHONY: all
all: $(BUILD_DIR)/$(IMAGE_NAME).iso

.PHONY: all-hdd
all-hdd: $(BUILD_DIR)/$(IMAGE_NAME).hdd

.PHONY: run
run: $(BUILD_DIR)/$(IMAGE_NAME).iso
	cd $(BUILD_DIR); \
	qemu-system-x86_64 -accel kvm -M q35 -m 2G -cdrom $(IMAGE_NAME).iso -boot d -serial stdio --smp 4

.PHONY: run-uefi
run-uefi: $(BUILD_DIR)/ovmf $(BUILD_DIR)/$(IMAGE_NAME).iso
	cd $(BUILD_DIR); \
	qemu-system-x86_64 -accel kvm -M q35 -m 2G -bios ovmf/OVMF.fd -cdrom $(IMAGE_NAME).iso -boot d -serial stdio --smp 4

.PHONY: run-hdd
run-hdd: $(BUILD_DIR)/$(IMAGE_NAME).hdd
	cd $(BUILD_DIR); \
	qemu-system-x86_64 -accel kvm -M q35 -m 2G -hda $(IMAGE_NAME).hdd -serial stdio --smp 4

.PHONY: run-hdd-uefi
run-hdd-uefi: $(BUILD_DIR)/ovmf $(IMAGE_NAME).hdd
	cd $(BUILD_DIR)
	qemu-system-x86_64 -accel kvm -M q35 -m 2G -bios ovmf/OVMF.fd -hda $(IMAGE_NAME).hdd -serial stdio --smp 4

$(BUILD_DIR)/ovmf:
	mkdir -p $(BUILD_DIR)/ovmf
	cd $(BUILD_DIR)/ovmf && curl -Lo OVMF.fd https://retrage.github.io/edk2-nightly/bin/RELEASEX64_OVMF.fd

$(BUILD_DIR)/limine/limine:
	if [ ! -d $(BUILD_DIR) ]; then mkdir -p $(BUILD_DIR); fi
	rm -rf $(BUILD_DIR)/limine
	curl -Lo $(BUILD_DIR)/limine.cfg https://github.com/limine-bootloader/limine-rust-template/raw/trunk/limine.cfg
	git clone https://github.com/limine-bootloader/limine.git $(BUILD_DIR)/limine --branch=v7.x-binary --depth=1
	$(MAKE) -C $(BUILD_DIR)/limine

.PHONY: kernel
kernel:
	cargo build --target ./x86_64.json
	cp -v target/x86_64/debug/hikari $(BUILD_DIR)/kernel

$(BUILD_DIR)/$(IMAGE_NAME).iso: $(BUILD_DIR)/limine/limine kernel
	cd $(BUILD_DIR); \
	rm -rf iso_root; \
	mkdir -p iso_root/boot; \
	cp -v kernel iso_root/boot/; \
	mkdir -p iso_root/boot/limine; \
	cp -v limine.cfg limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/boot/limine/; \
	mkdir -p iso_root/EFI/BOOT; \
	cp -v limine/BOOTX64.EFI iso_root/EFI/BOOT/; \
	cp -v limine/BOOTIA32.EFI iso_root/EFI/BOOT/; \
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso; \
	./limine/limine bios-install $(IMAGE_NAME).iso; \
	rm -rf iso_root

$(BUILD_DIR)/$(IMAGE_NAME).hdd: $(BUILD_DIR)/limine/limine kernel
	cd $(BUILD_DIR); \
	rm -f $(IMAGE_NAME).hdd; \
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(IMAGE_NAME).hdd; \
	sgdisk $(IMAGE_NAME).hdd -n 1:2048 -t 1:ef00; \
	./limine/limine bios-install $(IMAGE_NAME).hdd; \
	mformat -i $(IMAGE_NAME).hdd@@1M; \
	mmd -i $(IMAGE_NAME).hdd@@1M ::/EFI ::/EFI/BOOT ::/boot ::/boot/limine; \
	mcopy -i $(IMAGE_NAME).hdd@@1M kernel ::/boot; \
	mcopy -i $(IMAGE_NAME).hdd@@1M limine.cfg limine/limine-bios.sys ::/boot/limine; \
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTX64.EFI ::/EFI/BOOT; \
	mcopy -i $(IMAGE_NAME).hdd@@1M limine/BOOTIA32.EFI ::/EFI/BOOT

.PHONY: clean
clean:
	cd $(BUILD_DIR); \
	rm -rf iso_root $(IMAGE_NAME).iso $(IMAGE_NAME).hdd
	cargo clean

.PHONY: distclean
distclean: clean
	rm -rf build
	cargo clean