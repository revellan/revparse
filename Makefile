# PRODUCTION MAKEFILE

argparse:
	@cargo build --release

install: argparse
	@if [ "$(shell whoami)" != "root" ]; then \
		echo "You must be root to install this package!"; \
		exit 1; \
	fi
	@install -Dm755 target/release/argparse /usr/local/bin/argparse
	@echo "Installed to /usr/local/bin/argparse"

clean:
	@cargo clean

