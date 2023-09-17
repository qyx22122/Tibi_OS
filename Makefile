build:
	cargo build
	cargo bootimage

run:
	cargo build
	cargo bootimage
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-Tibi_OS/debug/bootimage-Tibi_OS.bin

lclean:
	rm -r target

wclean:
	rmdir /s /q target