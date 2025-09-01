# PRODUCTION MAKEFILE

revparse:
	@cargo build --release

install: revparse
	@if [ "$(shell whoami)" != "root" ]; then \
		echo "You must be root to install this package!"; \
		exit 1; \
	fi
	@install -Dm755 target/release/revparse /usr/local/bin/revparse
	@echo "Installed to /usr/local/bin/revparse"

clean:
	@cargo clean

