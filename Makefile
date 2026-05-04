BINARY_NAME := echo_dc
TARGET_DIR := target/release
INSTALL_PATH := /usr/local/bin

# Default target: build + install
all: install

build:
	cargo build --release

install: build
	install -m 755 $(TARGET_DIR)/$(BINARY_NAME) $(INSTALL_PATH)/$(BINARY_NAME)

uninstall:
	rm -f $(INSTALL_PATH)/$(BINARY_NAME)

clean:
	cargo clean

.PHONY: all build install uninstall clean