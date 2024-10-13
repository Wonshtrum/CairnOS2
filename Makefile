SRC_RS = $(shell find arch -type f) $(shell find src -type f)
SRC_ISO = grub.cfg
DIR = isodir
BIN = $(DIR)/boot/cairnos
ISO = cairnos.iso
QEMU = qemu-system-i386

all: $(ISO)

$(BIN): $(SRC_RS)
	cargo +nightly build --release
	mkdir -p $(DIR)/boot/
	cp target/x86/release/cairnos $(DIR)/boot/

$(ISO): $(BIN) $(SRC_ISO)
	mkdir -p $(DIR)/boot/grub
	cp grub.cfg $(DIR)/boot/grub/
	grub-mkrescue -o $(ISO) $(DIR)

.PHONY: run
run: $(ISO)
	$(QEMU) -cdrom $(ISO)

.PHONY: run_term
run_term: $(ISO)
	$(QEMU) -kernel $(BIN) -append "--nographic" --nographic

.PHONY: gdb
gdb: $(ISO)
	$(QEMU) -s -S -cdrom $(ISO)&
	gdb $(BIN) -x script.gdb

.PHONY: disas
disas: $(BIN)
	objdump -D -Mintel $(BIN)

.PHONY: re
re:
	touch src/main.rs
	rm -f $(ISO)
	rm -rf $(DIR)
	make

.PHONY: clean
clean:
	rm -f $(ISO)
	rm -rf $(DIR)
	cargo clean
