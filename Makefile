APP_NAME = mrt_splitter
TARGET_DIR = target/release
INSTALL_DIR = $(HOME)/.cargo/bin
BINARY_PATH = $(TARGET_DIR)/$(APP_NAME)

.PHONY: build install clean

build:
	cargo build --release

install: build
	@mkdir -p $(INSTALL_DIR)
	cp $(BINARY_PATH) $(INSTALL_DIR)/$(APP_NAME)
	@echo "Installed to $(INSTALL_DIR)/$(APP_NAME)"

clean:
	cargo clean

